import { createFileRoute, ErrorComponentProps, stripSearchParams, useLoaderData } from '@tanstack/react-router'
import { useSuspenseQuery } from '@tanstack/react-query'
import { Suspense, useState } from 'react'
import { zodValidator } from '@tanstack/zod-adapter'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { ListItem, ListRequest, ListRequestSchema, ListSortBy, ListSortOrder, MagnetResponseSchema, Mirror, MirrorRouteParamsSchema } from '@/lib/types'
import { queryClient } from '@/main'
import { cn } from '@/lib/utils'
import { ArrowDown, Download, Magnet } from 'lucide-react';
import { SizeCell } from '@/components/ui/size-cell'
import { NumberCell } from '@/components/ui/number-cell'
import { CategoryCell } from '@/components/ui/category-cell'
import { DateCell } from '@/components/ui/date-cell'
import { TitleCell } from '@/components/ui/title-cell'
import { Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink, PaginationNext, PaginationPrevious } from '@/components/ui/pagination'
import { Header } from '@/components/header'
import { Footer } from '@/components/footer'
import { Button } from '@/components/ui/button'
import { toast } from 'sonner'
import { ApiUrl } from '@/lib/url'
import { listQueryOptions } from '@/lib/query'
import { Skeleton } from '@/components/ui/skeleton'
import { ErrorCard } from '@/components/error'

export const Route = createFileRoute('/_proxy/$mirror/')({
  component: RouteComponent,
  pendingComponent: PendingComponent,
  errorComponent: ErrorComponent,
  parseParams: (params) => MirrorRouteParamsSchema.parse(params),
  validateSearch: zodValidator(ListRequestSchema),
  loaderDeps: ({ search }) => ({ search }),
  loader: async ({ deps: { search }, params }) => {
    const query = listQueryOptions(params.mirror, search)
    return await queryClient.ensureQueryData(query)
  },
  search: {
    middlewares: [stripSearchParams({
      p: 1,
      c: '0_0',
      s: 'id',
      o: 'desc',
      f: '0',
      q: '',
    })],
  }
})

function SortableHeader({
  sort_name,
  children,
  className,
  search
}: {
  sort_name: ListSortBy;
  children: React.ReactNode;
  className?: string;
  search?: ListRequest;
}) {
  const navigate = Route.useNavigate();
  const params = Route.useParams();
  const { s: sort_by, o: sort_order } = search ?? {};

  const isActive = sort_by === sort_name;
  const isAsc = isActive && sort_order === 'asc';
  const isDesc = isActive && sort_order === 'desc';

  const handleSort = () => {
    let sort_order: ListSortOrder | undefined = undefined;
    if (isActive) {
      if (isDesc) {
        sort_order = 'asc';
      } else {
        sort_order = 'desc';
      }
    } else {
      sort_order = 'desc';
    }

    navigate({
      to: '/$mirror',
      params,
      search: {
        ...search,
        s: sort_name,
        o: sort_order,
      },
      replace: true,
    });
  };

  return (
    <TableHead
      onClick={handleSort}
      className={cn("cursor-pointer select-none", isActive && "text-primary", className)}
    >
      <div className="flex items-center justify-center hover:underline">
        {children}
        {isActive && <ArrowDown className={cn("ml-1 h-4 w-4", isAsc && "rotate-180")} />}
      </div>
    </TableHead>
  );
}

function ItemRow({ mirror, item }: { mirror: Mirror, item: ListItem }) {
  const [magnetLoading, setMagnetLoading] = useState(false);

  const copyMagnetLink = async (e: React.MouseEvent) => {
    e.preventDefault();
    setMagnetLoading(true);

    const response = await fetch(`${ApiUrl}/api/mirror/${mirror.id}/magnet/${item.id}`);
    if (!response.ok) {
      toast.error('Failed to copy magnet link');
      setMagnetLoading(false);
      return;
    }

    let magnet_link = `unknown`;
    try {
      const data = MagnetResponseSchema.parse(await response.json());
      magnet_link = data.magnet_link;
    } catch (error) {
      const error_message = error instanceof Error ? error.message : 'Unknown error';
      toast.error('Failed to copy magnet link', { description: error_message });
      setMagnetLoading(false);
      return;
    }

    try {
      await navigator.clipboard.writeText(magnet_link);
      toast.success('Magnet link copied to clipboard', {
        description: magnet_link,
      });
    } catch {
      toast.error('Clipboard access denied', {
        description: 'However, you can still copy the magnet link manually: ' + magnet_link,
      });
    }
    setMagnetLoading(false);
  };

  return (
    <TableRow className={cn('hover:bg-muted/50', item.remake && 'bg-destructive/15', item.trusted && 'bg-success/15')}>
      <CategoryCell mirrorType={mirror.type} category={item.category} />
      <TitleCell id={item.id} title={item.title} />
      <SizeCell size={item.size} />
      <DateCell date={new Date(item.pub_date)} />
      <NumberCell variant="positive" value={item.seeders} />
      <NumberCell variant="negative" value={item.leechers} />
      <NumberCell value={item.downloads} />
      <NumberCell value={item.comments} />
      <TableCell className="w-[80px] text-center">
        <div className="flex">
          <Button variant="ghost" className="h-8 w-8 p-0">
            <a href={`${ApiUrl}/download/${item.id}`} className="text-primary hover:underline" target="_blank" rel="noopener noreferrer">
              <Download className="h-4 w-4" />
            </a>
          </Button>
          <Button variant="ghost" className="h-8 w-8 p-0" onClick={copyMagnetLink} disabled={magnetLoading}>
            <Magnet className={cn("h-4 w-4", magnetLoading && "animate-spin")} />
          </Button>
        </div>
      </TableCell>
    </TableRow>
  );
}


function ItemsTable({ mirror, search }: { mirror: Mirror, search: ListRequest }) {
  const { data: { items } } = useSuspenseQuery(listQueryOptions(mirror.id, search))

  return (
    <Table style={{ tableLayout: 'fixed' }}>
      <TableHeader>
        <TableRow>
          <TableHead className="w-[80px]">Category</TableHead>
          <TableHead className="truncate">Title</TableHead>
          <SortableHeader search={search} sort_name="size" className="w-[100px] text-right">Size</SortableHeader>
          <SortableHeader search={search} sort_name="id" className="w-[100px] text-center">Date</SortableHeader>
          <SortableHeader search={search} sort_name="seeders" className="w-[50px] text-center">S</SortableHeader>
          <SortableHeader search={search} sort_name="leechers" className="w-[50px] text-center">L</SortableHeader>
          <SortableHeader search={search} sort_name="downloads" className="w-[50px] text-center">D</SortableHeader>
          <SortableHeader search={search} sort_name="comments" className="w-[30px] text-center">C</SortableHeader>
          <TableHead className="w-[80px] text-center">Link</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {items.length === 0 ? (
          <TableRow>
            <TableCell colSpan={8} className="text-center">No items found</TableCell>
          </TableRow>
        ) : (
          items.map((item) => (
            <ItemRow key={item.id} mirror={mirror} item={item} />
          ))
        )}
      </TableBody>
    </Table>
  )
}


function TablePagination({ page, onPageChange, pageCount }: { page: number; onPageChange: (page: number) => void; pageCount: number }) {
  const generatePaginationItems = () => {
    const items = [];
    items.push(
      <PaginationItem key="page-1">
        <PaginationLink
          href="#"
          isActive={page === 1}
          onClick={(e) => {
            e.preventDefault();
            onPageChange(1);
          }}
        >
          1
        </PaginationLink>
      </PaginationItem>
    );

    if (page > 3) {
      items.push(
        <PaginationItem key="ellipsis-1">
          <PaginationEllipsis />
        </PaginationItem>
      );
    }

    const start = Math.max(2, page - 1);
    const end = Math.min(pageCount - 1, page + 1);
    for (let i = start; i <= end; i++) {
      if (i <= 1 || i >= pageCount) continue;
      items.push(
        <PaginationItem key={`page-${i}`}>
          <PaginationLink
            href="#"
            isActive={page === i}
            onClick={(e) => {
              e.preventDefault();
              onPageChange(i);
            }}
          >
            {i}
          </PaginationLink>
        </PaginationItem>
      );
    }

    if (page < pageCount - 2) {
      items.push(
        <PaginationItem key="ellipsis-2">
          <PaginationEllipsis />
        </PaginationItem>
      );
    }

    if (pageCount > 1) {
      items.push(
        <PaginationItem key={`page-${pageCount}`}>
          <PaginationLink
            href="#"
            isActive={page === pageCount}
            onClick={(e) => {
              e.preventDefault();
              onPageChange(pageCount);
            }}
          >
            {pageCount}
          </PaginationLink>
        </PaginationItem>
      );
    }

    return items;
  };

  return (<div className="mt-6 flex justify-center">
    <Pagination>
      <PaginationContent>
        <PaginationItem>
          <PaginationPrevious
            href="#"
            onClick={(e) => {
              e.preventDefault();
              onPageChange(page - 1);
            }}
            className={page <= 1 ? "pointer-events-none opacity-50" : ""}
          />
        </PaginationItem>

        {generatePaginationItems()}

        <PaginationItem>
          <PaginationNext
            href="#"
            onClick={(e) => {
              e.preventDefault();
              onPageChange(page + 1);
            }}
            className={page >= pageCount ? "pointer-events-none opacity-50" : ""}
          />
        </PaginationItem>
      </PaginationContent>
    </Pagination>
  </div>);
}

function RouteComponent() {
  const navigate = Route.useNavigate();
  const search = Route.useSearch();
  const { mirror: mirror_id } = Route.useParams();
  const data = useLoaderData({ from: '/_proxy' });
  const mirror = data.items.find((item) => item.id === mirror_id);
  if (!mirror) {
    navigate({ to: '/', replace: true });
    return null;
  }

  const pageCount = search.p + 5;
  const onPageChange = (page: number) => {
    if (page < 1 || page > pageCount) return;
    navigate({ search: { ...search, p: page } })
  }

  return (
    <div className="mx-auto container">
      <Header mirror_id={mirror_id} search={search} />
      <main className="container mx-auto">
        <div className="container mx-auto py-2">
          <Suspense fallback={<div>Loading...</div>}>
            <ItemsTable mirror={mirror} search={search} />
            <TablePagination
              page={search.p}
              onPageChange={onPageChange}
              pageCount={pageCount}
            />
          </Suspense>
        </div>
      </main>
      <Footer />
    </div>
  )
}

function ItemsTableSkeleton() {
  return (
    <Table style={{ tableLayout: 'fixed' }}>
      <TableHeader>
        <TableRow>
          <TableHead className="w-[80px]">Category</TableHead>
          <TableHead className="truncate">Title</TableHead>
          <SortableHeader sort_name="size" className="w-[100px] text-right">Size</SortableHeader>
          <SortableHeader sort_name="id" className="w-[100px] text-center">Date</SortableHeader>
          <SortableHeader sort_name="seeders" className="w-[50px] text-center">S</SortableHeader>
          <SortableHeader sort_name="leechers" className="w-[50px] text-center">L</SortableHeader>
          <SortableHeader sort_name="downloads" className="w-[50px] text-center">D</SortableHeader>
          <SortableHeader sort_name="comments" className="w-[30px] text-center">C</SortableHeader>
          <TableHead className="w-[80px] text-center">Link</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {Array.from({ length: 10 }, (_, index) => (
          <TableRow key={index}>
            <TableCell>
              <Skeleton className="h-4 w-full" />
            </TableCell>
            <TableCell >
              <Skeleton className="h-4 w-full" />
            </TableCell>
            <TableCell>
              <Skeleton className="h-4 w-full" />
            </TableCell>
            <TableCell>
              <Skeleton className="h-4 w-full" />
            </TableCell>
            <TableCell>
              <Skeleton className="h-4 w-full" />
            </TableCell>
            <TableCell>
              <Skeleton className="h-4 w-full" />
            </TableCell>
            <TableCell>
              <Skeleton className="h-4 w-full" />
            </TableCell>
            <TableCell>
              <Skeleton className="h-4 w-full" />
            </TableCell>
            <TableCell>
              <Skeleton className="h-4 w-full" />
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}

function TablePaginationSkeleton() {
  return (
    <div className="flex justify-center mt-6">
      <Skeleton className="h-8 w-32" />
    </div>
  )
}

function PendingComponent() {
  const navigate = Route.useNavigate();
  const search = Route.useSearch();
  const { mirror: mirror_id } = Route.useParams();
  const data = useLoaderData({ from: '/_proxy' });
  const mirror = data.items.find((item) => item.id === mirror_id);
  if (!mirror) {
    navigate({ to: '/', replace: true });
    return null;
  }

  return (
    <div className="mx-auto container">
      <Header mirror_id={mirror_id} search={search} />
      <main className="container mx-auto">
        <div className="container mx-auto py-2">
          <ItemsTableSkeleton />
          <TablePaginationSkeleton />
        </div>
      </main>
      <Footer />
    </div>
  )
}

function ErrorComponent({ error }: ErrorComponentProps) {
  const navigate = Route.useNavigate();
  const search = Route.useSearch();
  const { mirror: mirror_id } = Route.useParams();
  const data = useLoaderData({ from: '/_proxy' });
  const mirror = data.items.find((item) => item.id === mirror_id);
  if (!mirror) {
    navigate({ to: '/', replace: true });
    return null;
  }

  return (
    <div className="mx-auto container">
      <Header mirror_id={mirror_id} search={search} />
      <main className="container mx-auto">
        <div className="container mx-auto py-2">
          <ErrorCard error={error} title="An error occurred while loading the sites" onRetry={() => { window.location.reload() }} />
        </div>
      </main>
      <Footer />
    </div>
  )
}