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
import { useQuery } from '@tanstack/react-query'
import { Loader2 } from 'lucide-react'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'

export const Route = createFileRoute('/stats')({
  component: RouteComponent,
})

const torrentPerDaySchema = z.array(
  z.object({
    date: z.string(),
    count: z.number(),
  }),
);

const eventsSchema = z.array(
  z.object({
    url: z.string(),
    date: z.string(),
    event: z.string(),
    status: z.string(),
    rate_limited: z.boolean(),
  }),
);


function TorrentsPerDay() {

  const query = useQuery({
    queryKey: ['stats', 'torrent-per-day'],
    queryFn: async () => {
      const url = import.meta.env.VITE_API_URL
      const response = await fetch(`${url}api/stats/torrents-per-day`)
      if (!response.ok) {
        throw new Error('Failed to fetch stats')
      }
      const data = await response.json();
      const parsedData = torrentPerDaySchema.parse(data);
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
      <CardHeader className="flex flex-col items-stretch space-y-0 border-b p-0 sm:flex-row">
        <div className="flex flex-1 flex-col justify-center gap-1 px-6 py-5 sm:py-6">
          <CardTitle>Torrent Per Day</CardTitle>
          <CardDescription>
            The number of torrents added per day
          </CardDescription>
        </div>
      </CardHeader>
      <CardContent className="px-2 sm:p-6">
        <ChartContainer
          config={chartConfig}
          className="aspect-auto h-[250px] w-full"
        >
          <BarChart
            accessibilityLayer
            data={query.data}
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
      <CardHeader className="flex flex-col items-stretch space-y-0 border-b p-0 sm:flex-row">
        <div className="flex flex-1 flex-col justify-center gap-1 px-6 py-5 sm:py-6">
          <CardTitle>Events</CardTitle>
          <CardDescription>
            The number of events per day
          </CardDescription>
        </div>
      </CardHeader>
      <CardContent className="px-2 sm:p-6">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-[150px]">Date</TableHead>
              <TableHead className="w-[150px]">Event</TableHead>
              <TableHead className="w-[150px]">URL</TableHead>
              <TableHead className="w-[150px]">Status</TableHead>
              <TableHead className="w-[150px]">Rate Limited</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {(query.data ?? []).map((event) => (
              <TableRow key={event.date}>
                <TableCell>{event.date}</TableCell>
                <TableCell>{event.event}</TableCell>
                <TableCell>{event.url}</TableCell>
                <TableCell>{event.status}</TableCell>
                <TableCell>{event.rate_limited ? 'Yes' : 'No'}</TableCell>
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
    label: "Page Views",
  },
  count: {
    label: "Desktop",
    color: "hsl(var(--chart-1))",
  },
} satisfies ChartConfig

