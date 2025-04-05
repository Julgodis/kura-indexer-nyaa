import { mirrorQueryOptions } from '@/lib/query'
import { queryClient } from '@/main'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/_proxy')({
  component: RouteComponent,
  loader: async () => {
    return await queryClient.ensureQueryData(mirrorQueryOptions)
  },
})

function RouteComponent() {
  return <Outlet />
}
