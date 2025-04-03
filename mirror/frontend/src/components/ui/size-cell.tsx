import { TableCell } from "./table";

function sizeToString(size: number): string {
    const sizeInKib = size / 1024;
    const sizeInMiB = sizeInKib / 1024;
    const sizeInGiB = sizeInMiB / 1024;
    const sizeInTiB = sizeInGiB / 1024;
    const sizeInPiB = sizeInTiB / 1024;
    if (size < 1024) {
        return `${size} B`;
    } else if (sizeInKib < 1024) {
        return `${sizeInKib.toFixed(1)} KiB`;
    }
    else if (sizeInMiB < 1024) {
        return `${sizeInMiB.toFixed(1)} MiB`;
    }
    else if (sizeInGiB < 1024) {
        return `${sizeInGiB.toFixed(1)} GiB`;
    }
    else if (sizeInTiB < 1024) {
        return `${sizeInTiB.toFixed(1)} TiB`;
    }
    else {
        return `${sizeInPiB.toFixed(1)} PiB`;
    }
}

export function SizeCell({ size }: { size: number }) {
    return (<TableCell className="text-right">
        <span>{sizeToString(size)}</span>
    </TableCell>);
}
