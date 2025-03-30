"use client"

import { createFileRoute } from '@tanstack/react-router'
import { Bar, BarChart, CartesianGrid, XAxis } from "recharts"
import { z } from "zod"

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart"
import { queryOptions, useQuery, useSuspenseQuery } from '@tanstack/react-query'
import { Loader2 } from 'lucide-react'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { format } from 'date-fns'
import { queryClient, urlTransform } from '@/main'

const statsQuerryOptions = queryOptions({
  queryKey: ['stats', 'torrent-per-day'],
  queryFn: async () => {
    const response = await fetch(urlTransform('/api/stats/torrents-per-day'))
    if (!response.ok) {
      throw new Error('Failed to fetch stats')
    }
    const data = await response.json();
    const parsedData = torrentPerDaySchema.parse(data);
    return parsedData;
  },
})

export const Route = createFileRoute('/stats')({
  component: RouteComponent,
  pendingComponent: PendingComponent,
  errorComponent: ErrorComponent,
  loader: () => queryClient.ensureQueryData(statsQuerryOptions),
})

const torrentPerDaySchema = z.array(
  z.object({
    date: z.string(),
    count: z.number(),
  }),
);

const eventsSchema = z.array(
  z.object({
    date: z.string(),
    event_type: z.string(),
    event_data: z.any(),
  }),
);


function TorrentsPerDay() {

  const { data } = useSuspenseQuery(statsQuerryOptions)

  return (
    <Card>
      <CardHeader>
          <CardTitle>Torrent Per Day</CardTitle>
          <CardDescription>
            The number of torrents added per day
          </CardDescription>
      </CardHeader>
      <CardContent>
        <ChartContainer
          config={chartConfig}
          className="aspect-auto h-[250px] w-full"
        >
          <BarChart
            accessibilityLayer
            data={data}
            margin={{
              left: 12,
              right: 12,
            }}
          >
            <CartesianGrid vertical={false} />
            <XAxis
              dataKey="date"
              tickLine={false}
              axisLine={false}
              tickMargin={8}
              minTickGap={32}
              tickFormatter={(value) => {
                const date = new Date(value)
                return date.toLocaleDateString("en-US", {
                  month: "short",
                  day: "numeric",
                })
              }}
            />
            <ChartTooltip
              content={
                <ChartTooltipContent
                  className="w-[150px]"
                  nameKey="views"
                  labelFormatter={(value) => {
                    return new Date(value).toLocaleDateString("en-US", {
                      month: "short",
                      day: "numeric",
                      year: "numeric",
                    })
                  }}
                />
              }
            />
            <Bar dataKey="count" fill={`var(--color-count)`} />
          </BarChart>
        </ChartContainer>
      </CardContent>
    </Card>
  )
}

function Events() {
  const query = useQuery({
    queryKey: ['stats', 'events'],
    queryFn: async () => {
      const url = import.meta.env.VITE_API_URL
      const response = await fetch(`${url}api/stats/actions`)
      if (!response.ok) {
        throw new Error('Failed to fetch stats')
      }
      const data = await response.json();
      const parsedData = eventsSchema.parse(data);
      return parsedData;
    },
  })

  if (query.isLoading) {
    return (
      <div className="flex justify-center items-center min-h-[400px]">
        <Loader2 className="h-8 w-8 animate-spin" />
      </div>
    )
  }
  if (query.isError) {
    return (
      <Card>
        <CardContent className="pt-6">
          <div className="text-center text-destructive">
            Failed to load stats
          </div>
        </CardContent>
      </Card>
    )
  }

  return (
    <Card>
      <CardHeader>
          <CardTitle>Events</CardTitle>
          <CardDescription>
            The number of events per day
          </CardDescription>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-[150px]">Date</TableHead>
              <TableHead className="w-[150px]">Event</TableHead>
              <TableHead>Data</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {(query.data ?? []).map((event) => (
              <TableRow key={event.date}>
                <TableCell>{format(new Date(event.date), 'yyyy-MM-dd HH:mm:ss.SSS')}</TableCell>
                <TableCell>{event.event_type}</TableCell>
                <TableCell><pre>{JSON.stringify(event.event_data, null)}</pre></TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  )
}

function RouteComponent() {
  return (
    <div className="container py-6 space-y-6">
      <TorrentsPerDay />
      <Events />
    </div>
  )
}

const chartConfig = {
  views: {
    label: "Torrents",
  },
  count: {
    label: "Desktop",
    color: "hsl(var(--chart-1))",
  },
} satisfies ChartConfig


function PendingComponent() {
  return (
    <div className="flex justify-center items-center min-h-[400px]">
      <Loader2 className="h-8 w-8 animate-spin" />
    </div>
  )
}

function ErrorComponent(props: { error: Error }) {
  return (
    <div className="flex justify-center items-center min-h-[400px]">
      <Loader2 className="h-8 w-8 animate-spin" />
      <div className="text-center text-destructive">
        Failed to load stats
      </div>
      {props.error.message}
    </div>
  )
}

