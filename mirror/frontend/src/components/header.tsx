import { useSuspenseQuery } from '@tanstack/react-query';
import { Search } from './search';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './ui/select';
import { Route } from "@/routes/_proxy/$mirror";
import { mirrorQueryOptions } from '@/lib/query';
import nyaaLogoUrl from '@/assets/nyaa.png';

export function Header() {
    const navigate = Route.useNavigate();
    const { mirror } = Route.useParams();
    const { data } = useSuspenseQuery(mirrorQueryOptions);
    const currentMirror = data.items.find((item) => item.id === mirror);
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
                    <Search />
                </div>
            </div>
        </header>
    );
}