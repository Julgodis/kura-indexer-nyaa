import { cn } from '@/lib/utils';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '../ui/table';
import { ListSearch, Torrent, TorrentSort } from '@/types';
import { Link, useNavigate } from '@tanstack/react-router';
import { ArrowUpDown } from 'lucide-react'; // Assuming you use Lucide icons
import { format } from 'date-fns';

interface TorrentTableProps {
  torrents: Torrent[];
  search: ListSearch;
}


function SizeDisplay({ size }: { size: number }) {
  const sizeInKib = size / 1024;
  const sizeInMiB = sizeInKib / 1024;
  const sizeInGiB = sizeInMiB / 1024;
  const sizeInTiB = sizeInGiB / 1024;
  if (size < 1024) {
    return <span>{size} B</span>;
  } else if (sizeInKib < 1024) {
    return <span>{sizeInKib.toFixed(1)} KiB</span>;
  } else if (sizeInMiB < 1024) {
    return <span>{sizeInMiB.toFixed(1)} MiB</span>;
  } else if (sizeInGiB < 1024) {
    return <span>{sizeInGiB.toFixed(1)} GiB</span>;
  } else {
    return <span>{sizeInTiB.toFixed(1)} TiB</span>;
  }
}

function DateDisplay({ date }: { date: string }) {
  const formattedDate = format(new Date(date), 'MM-dd HH:mm');
  return <span>{formattedDate}</span>;
}

function TitleDisplay({ title }: { title: string }) {
  return <span>{title}</span>;
}

function SortableHeader({
  column,
  children,
  className,
  search
}: {
  column: TorrentSort | undefined;
  children: React.ReactNode;
  className?: string;
  search: ListSearch;
}) {
  const navigate = useNavigate();
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

function TorrentCategoryDisplay({ category_id }: { category_id: string }) {
  const categoryMap: Record<string, string> = {
    '0': 'All',
    '1': 'Anime',
    '2': 'Music',
    '3': 'Literature',
    '4': 'Live Action',
    '5': 'Pictures',
    '6': 'Software',
  };

  const subcategories: Record<string, string> = {
    '1_1': 'AMV',
    '1_2': 'English',
    '1_3': 'Non-English',
    '1_4': 'Raw',
    '2_1': 'Lossless',
    '2_2': 'Lossy',
    '3_1': 'English',
    '3_2': 'Non-English',
    '3_3': 'Raw',
    '4_1': 'English',
    '4_2': 'Idol/PV',
    '4_3': 'Non-English',
    '4_4': 'Raw',
    '5_1': 'Graphics',
    '5_2': 'Photos',
    '6_1': 'Apps',
    '6_2': 'Games',
  };

  const category = categoryMap[category_id.split('_')[0]];
  const subcategory = subcategories[category_id];

  if (!category) {
    return <span className="text-xs text-muted-foreground">Unknown Category</span>;
  } else if (!subcategory) {
    return <span>{category}</span>;
  } else {
    return <div className="flex flex-col">
      <span className='text-xs text-muted-foreground'>{categoryMap[category_id.split('_')[0]]}</span>
      <span className='text-xs'>{category_id in subcategories ? `${subcategories[category_id]}` : ''}</span>
    </div>
  }
}

export function TorrentTable({ torrents, search }: TorrentTableProps) {
  return (
    <div>
      <Table style={{ tableLayout: 'fixed' }}>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[80px]">Category</TableHead>
            <TableHead className="truncate">Name</TableHead>
            <SortableHeader search={search} column="size" className="w-[100px] text-right">Size</SortableHeader>
            <SortableHeader search={search} column="date" className="w-[100px] text-center">Date</SortableHeader>
            <SortableHeader search={search} column="seeders" className="w-[50px] text-center">S</SortableHeader>
            <SortableHeader search={search} column="leechers" className="w-[50px] text-center">L</SortableHeader>
            <SortableHeader search={search} column="downloads" className="w-[50px] text-center">C</SortableHeader>
          </TableRow>
        </TableHeader>
        <TableBody>
          {torrents.map((torrent) => (
            <TableRow key={torrent.guid} className={cn('hover:bg-muted/50', torrent.remake && 'bg-destructive/15', torrent.trusted && 'bg-success/15')}>
              <TableCell className="font-medium"><TorrentCategoryDisplay category_id={torrent.category_id} /></TableCell>
              <TableCell className="w-[300px] truncate">
                <Link to={torrent.link} className="text-primary hover:underline">
                  <TitleDisplay title={torrent.title} />
                </Link>
              </TableCell>
              <TableCell className="text-center"><SizeDisplay size={torrent.size} /></TableCell>
              <TableCell className="text-center"><DateDisplay date={torrent.pub_date} /></TableCell>
              <TableCell className="text-center text-emerald-600 font-medium">{torrent.seeders}</TableCell>
              <TableCell className="text-center text-destructive font-medium">{torrent.leechers}</TableCell>
              <TableCell className="text-center font-medium">{torrent.downloads}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
