import { Navigation } from './navigation';
import nyaaLogoUrl from '@/assets/nyaa.png';
import { useNavigate } from '@tanstack/react-router';

export function Header() {
    const navigate = useNavigate();

    return (
        <header>
            <div className="py-4">
                <div className="flex justify-between items-center gap-2 hover:cursor-pointer">
                    <img src={nyaaLogoUrl} alt="Logo" className="h-12 w-12" onClick={() => navigate({ to: '/' })} />
                    <h1 className="text-2xl font-bold" onClick={() => navigate({ to: '/' })}>Nyaa</h1>
                    <div className="flex-1" />
                    <Navigation />
                </div>
            </div>
        </header>
    );
}