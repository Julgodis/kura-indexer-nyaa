CREATE TABLE IF NOT EXISTS items (
    id TEXT NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    link TEXT NOT NULL,
    date DATETIME NOT NULL,
    seeders INTEGER NOT NULL,
    leechers INTEGER NOT NULL,
    downloads INTEGER NOT NULL,
    category TEXT NOT NULL, 
    size INTEGER NOT NULL,
    comments INTEGER NOT NULL,
    trusted INTEGER NOT NULL,
    remake INTEGER NOT NULL,
    download_link TEXT,
    magnet_link TEXT,

    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_items_date ON items(date DESC);

CREATE INDEX IF NOT EXISTS idx_items_category ON items(category);
CREATE INDEX IF NOT EXISTS idx_items_seeders ON items(seeders DESC);
CREATE INDEX IF NOT EXISTS idx_items_trusted_remake ON items(trusted, remake);
