import { useState } from 'react';
import { Input } from '../ui/input';
import { Button } from '../ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../ui/select';
import { ZoomIn } from 'lucide-react';
import { useNavigate } from '@tanstack/react-router';
import { TorrentCategory, TorrentFilter } from '@/types';

export function SearchForm() {
  const [term, setTerm] = useState('');
  const [category, setCategory] = useState<TorrentCategory>('0_0');
  const [filter, setFilter] = useState<TorrentFilter>('0');
  const navigate = useNavigate();

  const onSearch = () => {
    // Handle search logic here
    console.log('Searching with:', { category, filter });

    navigate({
      to: '/',
      search: {
        term: term,
        category,
        filter,
        sort: 'date',
        sort_order: 'desc',
        offset: 0,
        limit: 75,
      },
    });
  }

  return (
    <div className="rounded-lg shadow p-4 mb-6">
      <div className="flex gap-4 flex-wrap">
        <div className="flex-1 min-w-[200px]">
          <Input type="text" placeholder="Search..." value={term} onChange={(e) => setTerm(e.target.value)} />
        </div>
        <Select value={category} onValueChange={(value) => setCategory(value as TorrentCategory)}>
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder="Category" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="0_0">All Categories</SelectItem>
            <SelectItem value="1_0">Anime</SelectItem>
            <SelectItem value="2_0">Audio</SelectItem>
            <SelectItem value="3_0">Literature</SelectItem>
            <SelectItem value="4_0">Live Action</SelectItem>
          </SelectContent>
        </Select>
        <Select value={filter} onValueChange={(value) => setFilter(value as TorrentFilter)}>
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder="Filter" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="0">No filter</SelectItem>
            <SelectItem value="1">No remakes</SelectItem>
            <SelectItem value="2">Trusted only</SelectItem>
          </SelectContent>
        </Select>
        <Button onClick={onSearch}>
          <ZoomIn className="h-4 w-4 mr-2" />
          Search
        </Button>
      </div>
    </div>
  );
}
