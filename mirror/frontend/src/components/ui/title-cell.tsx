import { Link } from "@tanstack/react-router";
import { TableCell } from "./table";

export function TitleCell({ id, title }: { id: number, title: string }) {
    return (
        <TableCell className="text-sm text-left text-blue-500 hover:text-blue-500/80 hover:underline truncate">
            <Link to='/$mirror/view/$id' params={(prev) => ({ mirror: prev.mirror ?? 'unknown', id })} search={{}}>
                {title}
            </Link>
        </TableCell>
    );
}