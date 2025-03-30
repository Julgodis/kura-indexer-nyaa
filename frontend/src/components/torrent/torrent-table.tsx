import { cn } from '@/lib/utils';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '../ui/table';
import { Torrent, TorrentSort } from '@/types';
import { useNavigate } from '@tanstack/react-router';
import { ArrowUpDown } from 'lucide-react'; // Assuming you use Lucide icons
import { format } from 'date-fns';
import { Route } from '@/routes/_list.index';

interface TorrentTableProps {
  torrents: Torrent[];
}


function SizeDisplay({ size }: { size: number }) {
  const sizeInKib = size / 1024;
  const sizeInMiB = sizeInKib / 1024;
  const sizeInGiB = sizeInMiB / 1024;
  const sizeInTiB = sizeInGiB / 1024;
  if (size < 1024) {
    return <span>{size} B</span>;
  } else if (sizeInKib < 1024) {
    return <span>{sizeInKib.toFixed(2)} KiB</span>;
  } else if (sizeInMiB < 1024) {
    return <span>{sizeInMiB.toFixed(2)} MiB</span>;
  } else if (sizeInGiB < 1024) {
    return <span>{sizeInGiB.toFixed(2)} GiB</span>;
  } else {
    return <span>{sizeInTiB.toFixed(2)} TiB</span>;
  }
}

function DateDisplay({ date }: { date: string }) {
  const formattedDate = format(new Date(date), 'yyyy-MM-dd HH:mm');
  return <span>{formattedDate}</span>;
}

function TitleDisplay({ title }: { title: string }) {
  return <span>{title}</span>;
}

function SortableHeader({
  column,
  children,
  className
}: {
  column: TorrentSort | undefined;
  children: React.ReactNode;
  className?: string;
}) {
  const navigate = useNavigate();
  const search = Route.useSearch();
  const { sort, sort_order } = search;

  const isActive = sort === column;
  const isAsc = isActive && sort_order === 'asc';
  const isDesc = isActive && sort_order === 'desc';

  const handleSort = () => {
    let newSort = column;
    let newOrder: 'asc' | 'desc' | undefined;

    if (isActive) {
      if (isAsc) {
        newSort = undefined;
        newOrder = undefined;
      } else if (isDesc) {
        newOrder = 'asc';
      } else {
        newOrder = 'desc';
      }
    } else {
      newOrder = 'desc';
    }

    navigate({
      to: '/',
      search: {
        ...search,
        sort: newSort,
        sort_order: newOrder,
      },
      replace: true,
    });
  };

  return (
    <TableHead
      onClick={handleSort}
      className={cn("cursor-pointer select-none", isActive && "text-primary", className)}
    >
      <div className="flex items-center gap-2 justify-center">
        {children}
        {isActive && <ArrowUpDown className={cn("ml-1 h-4 w-4", isAsc && "rotate-180")} />}
      </div>
    </TableHead>
  );
}

export function TorrentTable({ torrents }: TorrentTableProps) {
  return (
    <div>
      <Table style={{ tableLayout: 'fixed' }}>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[150px]">Category</TableHead>
            <TableHead className="truncate">Name</TableHead>
            <SortableHeader column="size" className="w-[100px]">Size</SortableHeader>
            <SortableHeader column="date" className="w-[150px]">Date</SortableHeader>
            <SortableHeader column="seeders" className="w-[70px] text-center">S</SortableHeader>
            <SortableHeader column="leechers" className="w-[70px] text-center">L</SortableHeader>
            <SortableHeader column="downloads" className="w-[70px] text-center">C</SortableHeader>
          </TableRow>
        </TableHeader>
        <TableBody>
          {torrents.map((torrent) => (
            <TableRow key={torrent.guid} className={cn('hover:bg-muted/50', torrent.remake && 'bg-destructive/15', torrent.trusted && 'bg-success/15')}>
              <TableCell className="font-medium">{torrent.category}</TableCell>
              <TableCell className="w-[300px] truncate">
                <a href={torrent.link} className="text-primary hover:underline">
                  <TitleDisplay title={torrent.title} />
                </a>
              </TableCell>
              <TableCell><SizeDisplay size={torrent.size} /></TableCell>
              <TableCell><DateDisplay date={torrent.pub_date} /></TableCell>
              <TableCell className="text-center text-emerald-600 font-medium">{torrent.seeders}</TableCell>
              <TableCell className="text-center text-destructive font-medium">{torrent.leechers}</TableCell>
              <TableCell className="text-center">{torrent.downloads}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
