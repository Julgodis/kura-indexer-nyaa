import { ErrorCard } from '@/components/error'
import { Footer } from '@/components/footer'
import { Header } from '@/components/header'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { viewQueryOptions } from '@/lib/query'
import { ListRequestSchema, MirrorViewRouteParamsSchema, ViewComment, ViewFile, ViewResponse } from '@/lib/types'
import { queryClient } from '@/main'
import { useSuspenseQuery } from '@tanstack/react-query'
import { createFileRoute, ErrorComponentProps } from '@tanstack/react-router'
import { Loader2 } from 'lucide-react'
import { Suspense } from 'react'
import Markdown from "react-markdown"
import remarkGfm from 'remark-gfm'
import remarkBreaks from 'remark-breaks'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'

export const Route = createFileRoute('/_proxy/$mirror/view/$id')({
  component: RouteComponent,
  parseParams: (params) => MirrorViewRouteParamsSchema.parse(params),
  pendingComponent: PendingComponent,
  errorComponent: ErrorComponent,
  notFoundComponent: NotFoundComponent,
  loader: async ({ params }) => {
    const query = viewQueryOptions(params.mirror, params.id)
    return await queryClient.ensureQueryData(query)
  },
})

const markdownComponents = {
  h3: ({ children }: any) => <h3 className="text-lg font-semibold">{children}</h3>,
  table: ({ children }: any) => <Table>{children}</Table>,
  th: ({ children }: any) => <TableHead>{children}</TableHead>,
  thead: ({ children }: any) => <TableHeader>{children}</TableHeader>,
  tbody: ({ children }: any) => <TableBody>{children}</TableBody>,
  td: ({ children }: any) => <TableCell>{children}</TableCell>,
  tr: ({ children }: any) => <TableRow>{children}</TableRow>,
  br: () => <br />,
  p: ({ children }: any) => <p className="my-2">{children}</p>,
  strong: ({ children }: any) => <strong className="font-semibold">{children}</strong>,
  em: ({ children }: any) => <em className="italic">{children}</em>,
  a: ({ children, href }: any) => <a href={href}>{children}</a>,
  img: ({ src, alt }: any) => <img src={src} alt={alt} className="max-w-full h-auto" />,
  blockquote: ({ children }: any) => <blockquote className="border-l-4 pl-4 italic">{children}</blockquote>,
  ul: ({ children }: any) => <ul className="list-disc pl-5">{children}</ul>,
  ol: ({ children }: any) => <ol className="list-decimal pl-5">{children}</ol>,
  li: ({ children }: any) => <li className="my-1">{children}</li>,
  code: ({ children }: any) => <code className="p-1 rounded">{children}</code>,
}

function InfoCard({ view }: { view: ViewResponse }) {
  return (
    <div className="bg-card text-card-foreground rounded-lg border shadow-sm mb-4 p-4">
      <h1 className="text-xl font-bold mb-2">{view.title}</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <p><span className="font-semibold">Category:</span> {view.category}</p>
          <p><span className="font-semibold">Date:</span> {new Date(view.pub_date).toLocaleString()}</p>
          <p><span className="font-semibold">Size:</span> {formatFileSize(view.size)}</p>
        </div>
        <div>
          <p><span className="font-semibold">Seeders:</span> <span className="text-green-600">{view.seeders}</span></p>
          <p><span className="font-semibold">Leechers:</span> <span className="text-red-600">{view.leechers}</span></p>
          <p><span className="font-semibold">Downloads:</span> {view.downloads}</p>
        </div>
      </div>
      <div className="mt-4">
        {view.magnet_link && (
          <a
            href={view.magnet_link}
            className="bg-primary text-primary-foreground hover:bg-primary/90 rounded-md px-4 py-2 text-sm font-medium"
          >
            Download Magnet
          </a>
        )}
        {view.trusted && <span className="ml-2 bg-green-100 text-green-800 text-xs font-medium px-2.5 py-0.5 rounded">Trusted</span>}
        {view.remake && <span className="ml-2 bg-yellow-100 text-yellow-800 text-xs font-medium px-2.5 py-0.5 rounded">Remake</span>}
      </div>
    </div>
  )
}

function Description({ description_md }: { description_md: string }) {
  return (
    <div className="bg-card text-card-foreground rounded-lg border shadow-sm p-4">
      <Markdown remarkPlugins={[remarkGfm, remarkBreaks]} components={markdownComponents}>
        {description_md}
      </Markdown>
    </div>
  )
}

function Comments({ comments }: { comments: ViewComment[] }) {
  return (
    <div className="bg-card text-card-foreground rounded-lg border shadow-sm p-4">
      {comments.length === 0 ? (
        <p className="text-muted-foreground italic">No comments yet</p>
      ) : (
        <div className="space-y-4">
          {comments.map(comment => (
            <div key={comment.id} className="border-b pb-4 last:border-0">
              <div className="flex items-center gap-2 mb-2">
                {comment.avatar && (
                  <img src={comment.avatar} alt={comment.user} className="w-8 h-8 rounded-full" />
                )}
                <div>
                  <p className="font-semibold">{comment.user}</p>
                  <p className="text-xs text-muted-foreground">
                    {new Date(comment.date).toLocaleString()}
                    {comment.edited_date && ` (edited: ${new Date(comment.edited_date).toLocaleString()})`}
                  </p>
                </div>
              </div>
              <p className="whitespace-pre-line">{comment.content}</p>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}

function Files({ files }: { files: ViewFile[] }) {
  return (
    <div className="bg-card text-card-foreground rounded-lg border shadow-sm p-4">
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead className="bg-muted text-muted-foreground">
            <tr>
              <th className="text-left p-2">Filename</th>
              <th className="text-right p-2">Size</th>
            </tr>
          </thead>
          <tbody>
            {files.map(file => (
              <tr key={file.id} className="border-b last:border-0">
                <td className="p-2">{file.name}</td>
                <td className="p-2 text-right">{formatFileSize(file.size)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}

// Helper function to format file sizes
function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

function RouteComponent() {
  const { mirror: mirror_id, id: view_id } = Route.useParams();
  const { data: view } = useSuspenseQuery(viewQueryOptions(mirror_id, view_id))
  const search = ListRequestSchema.parse({});

  return (
    <div className="mx-auto container">
      <Header mirror_id={mirror_id} search={search} />
      <main className="container mx-auto">
        <div className="container mx-auto py-2">
          <Suspense fallback={<div>Loading...</div>}>
            <InfoCard view={view} />
            <Tabs defaultValue="description" className="w-full">
              <TabsList>
                <TabsTrigger value="description">Description</TabsTrigger>
                <TabsTrigger value="comments">Comments</TabsTrigger>
                <TabsTrigger value="files">Files</TabsTrigger>
              </TabsList>
              <TabsContent value="description">
                <Description description_md={view.description_md} />
              </TabsContent>
              <TabsContent value="comments">
                <Comments comments={view.comments} />
              </TabsContent>
              <TabsContent value="files">
                <Files files={view.files} />
              </TabsContent>
            </Tabs>
          </Suspense>
        </div>
      </main>
      <Footer />
    </div>
  )
}


function PendingComponent() {
  return <div className="flex justify-center items-center h-screen"><Loader2 className="animate-spin" /></div>
}

function ErrorComponent({ error }: ErrorComponentProps) {
  return (
    <ErrorCard error={error} title="An error occurred while loading the sites" onRetry={() => { window.location.reload() }} />
  )
}

function NotFoundComponent() {
  return (
    <div className="flex justify-center items-center h-screen">
      <div className="text-red-500">404 Not Found</div>
    </div>
  )
}
