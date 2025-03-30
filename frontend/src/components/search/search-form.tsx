import { useEffect, useState } from 'react';
import { Input } from '../ui/input';
import { Button } from '../ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../ui/select';
import { ZoomIn } from 'lucide-react';
import { useNavigate } from '@tanstack/react-router';
import { ListSearch, TorrentCategory, TorrentFilter } from '@/types';
import { Separator } from '../ui/separator';

export function SearchForm({ search }: { search: ListSearch }) {
  const [term, setTerm] = useState('');
  const [fieldTerm, setFieldTerm] = useState('');
  const [category, setCategory] = useState<TorrentCategory>('0_0');
  const [filter, setFilter] = useState<TorrentFilter>('0');
  const navigate = useNavigate();

  useEffect(() => {
    setTerm(search.term ?? '');
    setFieldTerm(search.term ?? '');
    setCategory(search.category ?? '0_0');
    setFilter(search.filter ?? '0');
  }
    , [search.term, search.category, search.filter]);

  const onSearch = (term: string, category: TorrentCategory, filter: TorrentFilter) => {
    let newTerm = term.trim();
    let newCategory = category;
    let newFilter = filter;

    const termChanged = newTerm !== search.term;
    const categoryChanged = newCategory !== search.category;
    const filterChanged = newFilter !== search.filter;

    // Only update the state if any of the values have changed
    if (termChanged || categoryChanged || filterChanged) {
      navigate({
        to: '/',
        search: {
          term: term,
          category,
          filter,
          sort: search.sort,
          sort_order: search.sort_order,
          offset: 0,
          limit: 75,
        },
      });
    }
  };

  useEffect(() => {
    onSearch(term, category, filter);
  }
    , [term, category, filter]);

  // Handle category change
  const handleCategoryChange = (value: string) => {
    const newCategory = value as TorrentCategory;
    setCategory(newCategory);
  };

  // Handle filter change
  const handleFilterChange = (value: string) => {
    const newFilter = value as TorrentFilter;
    setFilter(newFilter);
  };

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setTerm(fieldTerm);
    setCategory(category);
    setFilter(filter);
  }

  return (
    <div className="rounded-lg shadow pb-2">
      <form onSubmit={onSubmit}>
        <div className="flex gap-4 flex-wrap">
          <div className="flex-1 min-w-[200px]">
            <Input
              type="text"
              placeholder="Search..."
              value={fieldTerm}
              onChange={(e) => setFieldTerm(e.target.value)}
            />
          </div>
          <Select
            value={category}
            onValueChange={handleCategoryChange}
          >
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="Category" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="0_0">All Categories</SelectItem>
              <Separator />
              <SelectItem value="1_0">Anime</SelectItem>
              <SelectItem value="1_1">Anime - AMV</SelectItem>
              <SelectItem value="1_2">Anime - English</SelectItem>
              <SelectItem value="1_3">Anime - Non-English</SelectItem>
              <SelectItem value="1_4">Anime - Raw</SelectItem>
              <Separator />
              <SelectItem value="2_0">Audio</SelectItem>
              <SelectItem value="2_1">Audio - Lossless</SelectItem>
              <SelectItem value="2_2">Audio - Lossy</SelectItem>
              <Separator />
              <SelectItem value="3_0">Literature</SelectItem>
              <SelectItem value="3_1">Literature - English</SelectItem>
              <SelectItem value="3_2">Literature - Non-English</SelectItem>
              <SelectItem value="3_3">Literature - Raw</SelectItem>
              <Separator />
              <SelectItem value="4_0">Live Action</SelectItem>
              <SelectItem value="4_1">Live Action - English</SelectItem>
              <SelectItem value="4_2">Live Action - Idol/PV</SelectItem>
              <SelectItem value="4_3">Live Action - Non-English</SelectItem>
              <SelectItem value="4_4">Live Action - Raw</SelectItem>
              <Separator />
              <SelectItem value="5_0">Pictures</SelectItem>
              <SelectItem value="5_1">Pictures - Graphics</SelectItem>
              <SelectItem value="5_2">Pictures - Photos</SelectItem>
              <Separator />
              <SelectItem value="6_0">Software</SelectItem>
              <SelectItem value="6_1">Software - Apps</SelectItem>
              <SelectItem value="6_2">Software - Games</SelectItem>
            </SelectContent>
          </Select>
          <Select
            value={filter}
            onValueChange={handleFilterChange}
          >
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="Filter" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="0">No filter</SelectItem>
              <SelectItem value="1">No remakes</SelectItem>
              <SelectItem value="2">Trusted only</SelectItem>
            </SelectContent>
          </Select>
          <Button type="submit" variant="outline" className="flex items-center">
            <ZoomIn className="h-4 w-4 mr-2" />
            Search
          </Button>
        </div>
      </form>
    </div>
  );
}
