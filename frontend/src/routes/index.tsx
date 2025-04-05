import { ErrorCard } from '@/components/error';
import { mirrorQueryOptions } from '@/lib/query';
import { queryClient } from '@/main';
import { useSuspenseQuery } from '@tanstack/react-query';
import { createFileRoute, ErrorComponentProps } from '@tanstack/react-router'
import { Loader2 } from 'lucide-react';



export const Route = createFileRoute('/')({
  component: RouteComponent,
  pendingComponent: PendingComponent,
  errorComponent: ErrorComponent,
  notFoundComponent: NotFoundComponent,
  loader: async () => {
    return await queryClient.ensureQueryData(mirrorQueryOptions)
  },
})

function RouteComponent() {
  const navigate = Route.useNavigate();
  const { data } = useSuspenseQuery(mirrorQueryOptions)

  if (data.items.length === 0) {
    return <ErrorComponent error={new Error('No mirrors available')} reset={() => { }} />
  }

  navigate({ to: `/$mirror`, params: { mirror: data.items[0].id } })
  return (
    <div className="flex justify-center items-center h-screen">
      <div className="text-green-500">Redirecting to the first mirror...</div>
    </div>
  )
}


function PendingComponent() {
  return <div className="flex justify-center items-center h-screen"><Loader2 className="animate-spin" /></div>
}

function ErrorComponent({ error }: ErrorComponentProps) {
  return (
    <ErrorCard error={error} title="An error occurred while loading the sites" onRetry={() => window.location.reload()} />
  )
}

function NotFoundComponent() {
  return (
    <div className="flex justify-center items-center h-screen">
      <div className="text-red-500">404 Not Found</div>
    </div>
  )
}
