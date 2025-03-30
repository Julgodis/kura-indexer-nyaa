import { TorrentPagination } from '@/components/torrent/torrent-pagination';
import { TorrentTable } from '@/components/torrent/torrent-table';
import { queryOptions, useSuspenseQuery } from '@tanstack/react-query';
import { createFileRoute, ErrorComponentProps } from '@tanstack/react-router'
import { zodValidator } from '@tanstack/zod-adapter'
import { ListSearch, ListSearchSchema, TorrentListResponseSchema } from '@/types';
import { queryClient, urlTransform } from '@/main';
import { Loader2 } from 'lucide-react';
import { SearchForm } from '@/components/search/search-form';

const torrentsQueryOptions = ({
  term,
  category,
  filter,
  sort,
  sort_order,
  offset,
  limit,
}: ListSearch) => queryOptions({
  queryKey: ['torrents', term, category, filter, sort, sort_order, offset, limit],
  queryFn: async () => {
    const response = await fetch(urlTransform("/api/torrents"), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        term,
        category,
        filter,
        sort,
        sort_order,
        offset,
        limit,
      }),
    });

    const data = await response.json();
    const parsedData = TorrentListResponseSchema.parse(data);
    return parsedData;
  },
});

export const Route = createFileRoute('/')({
  component: RouteComponent,
  errorComponent: ErrorComponent,
  pendingComponent: PendingComponent,
  validateSearch: zodValidator(ListSearchSchema),
  loaderDeps: ({ search }) => ({ search }),
  loader: async ({ deps: { search } }) => await queryClient.ensureQueryData(torrentsQueryOptions(search)),
})

function RouteComponent() {
  const search = Route.useSearch();
  const navigate = Route.useNavigate();

  const { data, isFetching } = useSuspenseQuery(torrentsQueryOptions(search));

  if (isFetching) {
    return <>
      <div className="flex justify-center items-center min-h-[400px]">
        <Loader2 className="h-8 w-8 animate-spin" />
      </div>
    </>;
  }

  const limit = search.limit || 75;
  const offset = search.offset || 0;

  const mockTorrents = data.torrents;
  const currentPage = Math.floor(offset / limit) + 1;
  const totalPages = Math.ceil(data.total / limit);
  const onPageChange = (page: number) => {
    const newOffset = (page - 1) * limit;
    console.log('Page changed to:', page, 'Offset:', newOffset);
    navigate({
      to: '/',
      search: {
        ...search,
        offset: newOffset,
      },
    });
  };

  return <>
    <SearchForm search={search} />
    <TorrentTable search={search} torrents={mockTorrents} />
    <TorrentPagination
      currentPage={currentPage}
      totalPages={totalPages}
      onPageChange={onPageChange}
    />
  </>;
}

function PendingComponent() {
  const search = Route.useSearch();
  return (
    <><SearchForm search={search} />
      <div className="flex justify-center items-center min-h-[400px]">
        <Loader2 className="h-8 w-8 animate-spin" />
      </div>
    </>
  );
}

function ErrorComponent(props: ErrorComponentProps) {
  const search = Route.useSearch();
  return (
    <><SearchForm search={search} /> <div className="flex justify-center items-center min-h-[400px]">
      <Loader2 className="h-8 w-8 animate-spin" />
      <div className="text-center text-destructive">
        Failed to load torrents
      </div>
      <div className="text-sm text-muted-foreground">
        {props.error.message}
      </div>
    </div></>
  );
}
