import { FooterSkeleton } from '@/components/footer'
import { HeaderSkeleton } from '@/components/header'
import { mirrorQueryOptions } from '@/lib/query'
import { queryClient } from '@/main'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/_proxy')({
  component: RouteComponent,
  pendingComponent: PendingComponent,
  loader: async () => {
    return await queryClient.ensureQueryData(mirrorQueryOptions)
  },
})

function RouteComponent() {
  return <Outlet />
}

function PendingComponent() {
  return (
    <div className="mx-auto container">
      <HeaderSkeleton />
      <main className="container mx-auto">
        <div className="container mx-auto py-2">
          
        </div>
      </main>
      <FooterSkeleton />
    </div>)
}