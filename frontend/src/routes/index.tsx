import { SearchForm } from '@/components/search/search-form';
import { TorrentPagination } from '@/components/torrent/torrent-pagination';
import { TorrentTable } from '@/components/torrent/torrent-table';
import { useQuery } from '@tanstack/react-query';
import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { zodValidator } from '@tanstack/zod-adapter'
import { TorrentCategorySchema, TorrentFilterSchema, TorrentListResponseSchema, TorrentSortOrderSchema, TorrentSortSchema } from '@/types';

const torrentParamSchema = z.object({
  term: z.string().optional(),
  category: TorrentCategorySchema.optional(),
  filter: TorrentFilterSchema.optional(),
  sort: TorrentSortSchema.optional(),
  sort_order: TorrentSortOrderSchema.optional(),
  offset: z.number().optional(),
  limit: z.number().optional(),
});

export const Route = createFileRoute('/')({
  component: RouteComponent,
  errorComponent: () => <div>Error</div>,
  validateSearch: zodValidator(torrentParamSchema),
})

function RouteComponent() {
  const search = Route.useSearch();
  const navigate = Route.useNavigate();

  const url = import.meta.env.VITE_API_URL;
  const query = useQuery({
    queryKey: ['torrents', search.term, search.category, search.filter, search.sort, search.sort_order, search.offset, search.limit],
    queryFn: async () => {
      const response = await fetch(`${url}api/torrents`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          term: search.term,
          category: search.category,
          filter: search.filter,
          sort: search.sort,
          sort_order: search.sort_order,
          offset: search.offset,
          limit: search.limit,
        }),
      });

      const data = await response.json();
      const parsedData = TorrentListResponseSchema.parse(data);
      return parsedData;
    },
    initialData: { torrents: [], offset: 0, count: 0, total: 0 },
  });

  if (query.isLoading) {
    return <div>Loading...</div>;
  }
  if (query.isError) {
    return <div>Error: {query.error.message}</div>;
  }

  const limit = search.limit || 75;
  const offset = search.offset || 0;

  const mockTorrents = query.data.torrents;
  const currentPage = Math.floor(offset / limit) + 1;
  const totalPages = Math.ceil(query.data.total / limit);
  const onPageChange = (page: number) => {
    const newOffset = (page - 1) * limit;
    navigate({
      to: '/',
      search: {
        ...search,
        offset: newOffset,
      },
    });
  };

  return <>
    <SearchForm />
    <TorrentTable torrents={mockTorrents} />
    <TorrentPagination
      currentPage={currentPage}
      totalPages={totalPages}
      onPageChange={onPageChange}
    />

  </>;
} 
