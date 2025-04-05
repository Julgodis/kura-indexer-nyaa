import { ErrorCard } from '@/components/error';
import { Table, TableBody, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { healthQueryOptions } from '@/lib/query';
import { queryClient } from '@/main';
import { useSuspenseQuery } from '@tanstack/react-query';
import { createFileRoute, ErrorComponentProps } from '@tanstack/react-router'
import { Loader2 } from 'lucide-react';
import nyaaLogoUrl from '@/assets/nyaa.png';

export const Route = createFileRoute('/health')({
    component: RouteComponent,
    pendingComponent: PendingComponent,
    errorComponent: ErrorComponent,
    notFoundComponent: NotFoundComponent,
    loader: async () => {
        return await queryClient.ensureQueryData(healthQueryOptions)
    },
})

function RouteComponent() {
    const { data } = useSuspenseQuery(healthQueryOptions)


    return (
        <div className="flex justify-center items-center h-screen">
            {data.mirrors.map((mirror) => (
                <div key={mirror.id} className="flex flex-col items-center">
                    <img src={nyaaLogoUrl} alt="Logo" className="h-12 w-12" />
                    <div className="text-green-500">{mirror.name}</div>
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>Date</TableHead>
                                <TableHead>Path</TableHead>
                                <TableHead>Success</TableHead>
                                <TableHead>Cached</TableHead>
                                <TableHead>Response Time</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {mirror.requests.map((x) => (
                                <TableRow key={x[0]}>
                                    <TableHead>{new Date(x[0]).toLocaleString()}</TableHead>
                                    <TableHead>{x[1]}</TableHead>
                                    <TableHead>{x[2] ? 'Yes' : 'No'}</TableHead>
                                    <TableHead>{x[3] ? 'Yes' : 'No'}</TableHead>
                                    <TableHead>{(x[4] * 1000.0).toFixed(2)} ms</TableHead>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                </div>
            ))}
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
