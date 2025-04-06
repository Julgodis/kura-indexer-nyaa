import { Search } from './search';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './ui/select';
import nyaaLogoUrl from '@/assets/nyaa.png';
import { useLoaderData, useNavigate } from '@tanstack/react-router';
import { ListRequest } from '@/lib/types';

export function Header({ mirror_id,
    search,
}: { mirror_id: string, search: ListRequest }) {
    const data = useLoaderData({ from: '/_proxy' });
    const navigate = useNavigate();
    const currentMirror = data.items.find((item) => item.id === mirror_id);
    if (!currentMirror) {
        navigate({ to: '/', replace: true });
        return null;
    }

    return (
        <header>
            <div className="py-4">
                <div className="flex justify-between items-center gap-2 hover:cursor-pointer">
                    <div className="flex items-center gap-2 hover:cursor-pointer" onClick={() => navigate({ to: '/$mirror', params: { mirror: currentMirror.id } })}>
                        <img src={nyaaLogoUrl} alt="Logo" className="h-12 w-12" />
                    </div>
                    <Select
                        defaultValue={currentMirror.id}
                        onValueChange={(value) => {
                            if (value !== currentMirror.id) {
                                navigate({ to: `/$mirror`, params: { mirror: value } });
                            }
                        }}
                    >
                        <SelectTrigger>
                            {currentMirror.hidden ? (
                                <span className="text-gray-500">{currentMirror.name}</span>
                            ) : (
                                <SelectValue />
                            )}

                        </SelectTrigger>
                        <SelectContent>
                            {data.items
                                .filter((item) => !item.hidden)
                                .map((mirror) => (
                                    <SelectItem key={mirror.id} value={mirror.id}>
                                        {mirror.name}
                                    </SelectItem>
                                ))}
                        </SelectContent>
                    </Select>
                    <Search mirror_id={mirror_id} search={search} />
                </div>
            </div>
        </header>
    );
}

export function HeaderSkeleton() {
    return (
        <header>
            <div className="py-4">
                <div className="flex justify-between items-center gap-2">
                    <div className="flex items-center gap-2">
                        <img src={nyaaLogoUrl} alt="Logo" className="h-12 w-12" />
                    </div>
                    <Select disabled defaultValue="loading">
                        <SelectTrigger>
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="loading">Loading...</SelectItem>
                        </SelectContent>
                    </Select>
                    <div className="flex-1" />
                </div>
            </div>
        </header >
    );
}
