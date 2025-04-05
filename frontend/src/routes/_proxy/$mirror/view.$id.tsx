import { MirrorViewRouteParamsSchema } from '@/lib/types'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_proxy/$mirror/view/$id')({
  component: RouteComponent,
  parseParams: (params) => MirrorViewRouteParamsSchema.parse(params),
})

function RouteComponent() {
  return <div>Hello "/view/$id"!</div>
}
