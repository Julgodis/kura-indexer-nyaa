import { listCategories, ListCategory, listCategoryOrder, ListFilter, ListFilterSchema, listFilterValueToLabel } from "@/lib/types";
import { useState } from "react";
import { Input } from "./ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "./ui/select";
import { Button } from "./ui/button";
import { Route } from "@/routes/_proxy/$mirror";
import { Separator } from "./ui/separator";

export function Search() {
    const navigate = Route.useNavigate();
    const search = Route.useSearch();
    const [searchInput, setSearchInput] = useState(search.q || '');
    const [filter, setFilter] = useState(search.f);
    const [category, setCategory] = useState(search.c);

    const handleSearch = (e: React.FormEvent) => {
        e.preventDefault()
        navigate({ search: { ...search, q: searchInput } })
    }

    const onFilterChange = (value: string) => {
        const newFilter = value as ListFilter
        setFilter(newFilter)
        navigate({ search: { ...search, q: searchInput, f: newFilter } })
    }

    const onCategoryChange = (value: string) => {
        const newCategory = value as ListCategory
        setCategory(newCategory)
        navigate({ search: { ...search, q: searchInput, c: newCategory } })
    }

    const categoryElements = [];

    for (let i = 0; i < listCategoryOrder.length; i++) {
        const category_id = listCategoryOrder[i];
        const category = listCategories[category_id];

        if (i > 0) {
            categoryElements.push(<Separator key={`sep_${category.id}`} />);
        }

        categoryElements.push(
            <SelectItem key={category.id} value={category.id}>
                {category.name}
            </SelectItem>
        )

        category.order.forEach((subcategory_id) => {
            const subcategory = category.subcategories[subcategory_id];
            categoryElements.push(
                <SelectItem key={subcategory.id} value={subcategory.id}>
                    {`${category.name} - ${subcategory.name}`}
                </SelectItem>
            )
        })
    }

    return (
        <form onSubmit={handleSearch} className="flex flex-1 gap-2 items-center">
            <Input
                placeholder="Search..."
                value={searchInput}
                onChange={(e) => setSearchInput(e.target.value)}
                className="min-w-sm flex-1"
            />
            <Select
                value={category}
                onValueChange={onCategoryChange}
            >
                <SelectTrigger className="w-[180px]">
                    <SelectValue placeholder="Category" />
                </SelectTrigger>
                <SelectContent>
                    {...categoryElements}
                </SelectContent>
            </Select>
            <Select
                value={filter}
                onValueChange={onFilterChange}
            >
                <SelectTrigger className="w-[180px]">
                    <SelectValue placeholder="Filter" />
                </SelectTrigger>
                <SelectContent>
                    {ListFilterSchema.options.map((option) => (
                        <SelectItem key={option} value={option}>
                            {listFilterValueToLabel(option)}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
            <Button type="submit">Search</Button>
        </form>

    )
}