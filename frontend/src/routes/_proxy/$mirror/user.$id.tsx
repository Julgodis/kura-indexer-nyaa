import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/_proxy/$mirror/user/$id')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/user/$id"!</div>
}
