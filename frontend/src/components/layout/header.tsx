import { Navigation } from './navigation';
import nyaaLogoUrl from '@/assets/nyaa.png';

export function Header() {
    return (
        <header>
            <div className="py-4">
                <div className="flex justify-between items-center gap-2">
                    <img src={nyaaLogoUrl} alt="Logo" className="h-12 w-12" />
                    <h1 className="text-2xl font-bold">Nyaa</h1>
                    <div className="flex-1" />
                    <Navigation />
                </div>
            </div>
        </header>
    );
}