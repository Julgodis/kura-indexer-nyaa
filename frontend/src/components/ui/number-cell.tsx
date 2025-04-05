import { cn } from "@/lib/utils";
import { TableCell } from "./table";


export function NumberCell({ value, variant = "default" }: { value: number, variant?: 'default' | 'positive' | 'negative' }) {
    let color = undefined;
    if (variant === 'positive') {
        color = 'text-emerald-600';
    } else if (variant === 'negative') {
        color = 'text-destructive';
    }

    return (
        <TableCell className={cn('text-sm', 'text-center', color)}>
            <span>
                {value.toLocaleString()}
            </span>
        </TableCell>
    );
}
