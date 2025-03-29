import { Navigation } from './navigation';

export function Header() {
    return (
        <header className="border-b">
            <div className="py-4">
                <div className="flex justify-between items-center">
                    <h1 className="text-2xl font-bold">くら Nyaa Indexer</h1>
                    <div className="flex-1" />
                    <Navigation />
                </div>
            </div>
        </header>
    );
}