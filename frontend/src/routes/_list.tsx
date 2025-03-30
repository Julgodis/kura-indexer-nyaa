import { createFileRoute, Outlet } from '@tanstack/react-router'
import { SearchForm } from '@/components/search/search-form';

export const Route = createFileRoute('/_list')({
  component: RouteComponent,
})

function RouteComponent() {
  return <>
    <SearchForm />
    <Outlet />
  </>;
}
