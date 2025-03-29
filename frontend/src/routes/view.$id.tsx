import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { Loader2 } from 'lucide-react'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Avatar } from '@/components/ui/avatar'
import { Separator } from '@/components/ui/separator'
import { formatDistanceToNow } from 'date-fns'
import { TorrentResponse } from '@/types'
import Markdown from "react-markdown"
import remarkGfm from 'remark-gfm'
import remarkBreaks from 'remark-breaks'

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
  a: ({ children, href }: any) => <a href={href} className="text-blue-500 hover:underline">{children}</a>,
  img: ({ src, alt }: any) => <img src={src} alt={alt} className="max-w-full h-auto" />,
  blockquote: ({ children }: any) => <blockquote className="border-l-4 pl-4 italic">{children}</blockquote>,
  ul: ({ children }: any) => <ul className="list-disc pl-5">{children}</ul>,
  ol: ({ children }: any) => <ol className="list-decimal pl-5">{children}</ol>,
  li: ({ children }: any) => <li className="my-1">{children}</li>,
  code: ({ children }: any) => <code className="bg-gray-100 p-1 rounded">{children}</code>,
}

// API function to fetch torrent data
async function fetchTorrent(id: string): Promise<TorrentResponse> {
  const url = import.meta.env.VITE_API_URL;
  const response = await fetch(`${url}/api/torrent/${id}`)
  if (!response.ok) {
    throw new Error('Failed to fetch torrent')
  }
  return response.json()
}

export const Route = createFileRoute('/view/$id')({
  loader: ({ params }) => ({
    torrentId: params.id
  }),
  component: RouteComponent,
})

function RouteComponent() {
  const { torrentId } = Route.useLoaderData()

  const { data: torrent, isLoading, error } = useQuery({
    queryKey: ['torrent', torrentId],
    queryFn: () => fetchTorrent(torrentId),
  })

  if (isLoading) {
    return (
      <div className="flex justify-center items-center min-h-[400px]">
        <Loader2 className="h-8 w-8 animate-spin" />
      </div>
    )
  }

  if (error || !torrent) {
    return (
      <Card>
        <CardContent className="pt-6">
          <div className="text-center text-destructive">
            Failed to load torrent data
          </div>
        </CardContent>
      </Card>
    )
  }

  return (
    <div className="container py-6 space-y-6">
      {/* Torrent header */}
      <Card>
        <CardHeader>
          <div className="flex justify-between items-start">
            <div>
              <CardTitle className="text-2xl">{torrent.title}</CardTitle>
              <CardDescription className="mt-2 flex gap-2">
                <Badge variant="outline">{torrent.category}</Badge>
                <span>Uploaded {formatDistanceToNow(new Date(torrent.pub_date))} ago by {torrent.uploader}</span>
              </CardDescription>
            </div>
            <a
              href={torrent.magnet_link}
              className="bg-primary hover:bg-primary/90 text-primary-foreground px-4 py-2 rounded-md"
            >
              Magnet Download
            </a>
          </div>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div>
              <p className="text-sm text-muted-foreground">Size</p>
              <p className="font-medium">{torrent.size}</p>
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Seeders</p>
              <p className="font-medium text-green-600">{torrent.seeders}</p>
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Leechers</p>
              <p className="font-medium text-red-600">{torrent.leechers}</p>
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Downloads</p>
              <p className="font-medium">{torrent.downloads}</p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Torrent details */}
      <Tabs defaultValue="description" className="w-full">
        <TabsList>
          <TabsTrigger value="description">Description</TabsTrigger>
          <TabsTrigger value="files">Files</TabsTrigger>
          <TabsTrigger value="comments">Comments</TabsTrigger>
        </TabsList>

        <TabsContent value="description" className="mt-4">
          <Card>
            <CardContent className="pt-6">
              <div
                className="prose dark:prose-invert max-w-none markdown"
              >
                <Markdown remarkPlugins={[remarkGfm, remarkBreaks]} components={markdownComponents}>
                  {torrent.description_markdown}
                </Markdown>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="files" className="mt-4">
          <Card>
            <CardContent className="pt-6">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>File Name</TableHead>
                    <TableHead className="text-right">Size</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {torrent.files.map((file, index) => (
                    <TableRow key={index}>
                      <TableCell className="font-medium">{file.name}</TableCell>
                      <TableCell className="text-right">{file.size}</TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="comments" className="mt-4">
          <Card>
            <CardContent className="pt-6">
              {torrent.comments.length === 0 ? (
                <p className="text-center text-muted-foreground py-4">No comments yet</p>
              ) : (
                <div className="space-y-6">
                  {torrent.comments.map((comment) => (
                    <div key={comment.id} className="space-y-2">
                      <div className="flex items-center gap-2">
                        <Avatar>
                          <img src={comment.avatar} alt={comment.user} />
                        </Avatar>
                        <div>
                          <p className="font-medium">{comment.user}</p>
                          <p className="text-sm text-muted-foreground">
                            {formatDistanceToNow(new Date(comment.date))} ago
                          </p>
                        </div>
                      </div>
                      <p className="pl-10">{comment.content}</p>
                      <Separator className="mt-4" />
                    </div>
                  ))}
                </div>
              )}
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  )
}
