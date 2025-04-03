import { TableCell } from "./table";
import { format } from 'date-fns';

export function DateCell({ date }: { date: Date }) {
    return (
        <TableCell className="text-sm text-center">
            <span>
                {format(date, 'yyyy-MM-dd')}
            </span>
        </TableCell>
    );
}
