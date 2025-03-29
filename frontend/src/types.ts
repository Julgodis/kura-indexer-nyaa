import { z } from "zod";

export const TorrentCategorySchema = z.enum([
    "0_0",
    "1_0",
    "2_0",
    "3_0",
    "4_0",
    "5_0",
    "6_0",
]);

export const TorrentFilterSchema = z.enum([
    "0",
    "1",
    "2",
]);

export const TorrentSortSchema = z.enum([
    "date",
    "seeders",
    "leechers",
    "downloads",
    "size",
]);

export const TorrentSortOrderSchema = z.enum([
    "asc",
    "desc",
]);

export const TorrentListRequestSchema = z.object({
    term: z.string().optional(),
    category: TorrentCategorySchema.optional(),
    filter: TorrentFilterSchema.optional(),
    sort: TorrentSortSchema.optional(),
    sort_order: TorrentSortOrderSchema.optional(),
    offset: z.number().optional(),
    limit: z.number().optional(),
});

export const TorrentSchema = z.object({
    title: z.string(),
    link: z.string(),
    guid: z.string(),
    pub_date: z.string(),
    seeders: z.number(),
    leechers: z.number(),
    downloads: z.number(),
    info_hash: z.string(),
    category_id: z.string(),
    category: z.string(),
    size: z.number(), // For large numbers, might need z.bigint() if browser support is needed
    comments: z.number(),
    trusted: z.boolean(),
    remake: z.boolean(),
    description: z.string().optional(),
    magnet_link: z.string().optional(),
    downoad_link: z.string().optional(),
});

export const TorrentListResponseSchema = z.object({
    torrents: z.array(TorrentSchema),
    offset: z.number(),
    count: z.number(),
    total: z.number(),
});

export const TorrentRequestSchema = z.object({
    id: z.string(),
});

export const TorrentResponseSchema = z.object({
    title: z.string(),
    link: z.string(),
    guid: z.string(),
    pub_date: z.string(),
    seeders: z.number(),
    leechers: z.number(),
    downloads: z.number(),
    info_hash: z.string(),
    category_id: z.string(),
    category: z.string(),
    size: z.number(), // For large numbers, might need z.bigint() if browser support is needed
    trusted: z.boolean(),
    remake: z.boolean(),
    description: z.string(),
    description_markdown: z.string(),
    uploader: z.string(),
    magnet_link: z.string(),
    downoad_link: z.string(),
    files: z.array(z.object({
        name: z.string(),
        size: z.string(),
    })),
    comments: z.array(z.object({
        id: z.string(),
        user: z.string(),
        avatar: z.string(),
        date: z.string(),
        content: z.string(),
    })),
});

// Type definitions derived from the schemas
export type TorrentListRequest = z.infer<typeof TorrentListRequestSchema>;
export type Torrent = z.infer<typeof TorrentSchema>;
export type TorrentListResponse = z.infer<typeof TorrentListResponseSchema>;
export type TorrentRequest = z.infer<typeof TorrentRequestSchema>;
export type TorrentResponse = z.infer<typeof TorrentResponseSchema>;

export type TorrentCategory = z.infer<typeof TorrentCategorySchema>;
export type TorrentFilter = z.infer<typeof TorrentFilterSchema>;
export type TorrentSort = z.infer<typeof TorrentSortSchema>;
export type TorrentSortOrder = z.infer<typeof TorrentSortOrderSchema>;
