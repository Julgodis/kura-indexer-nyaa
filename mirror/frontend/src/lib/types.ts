import { RecordType, z } from "zod";

export const ListCategorySchema = z.enum([
    "0_0",
    "1_0",
    "1_1",
    "1_2",
    "1_3",
    "1_4",
    "2_0",
    "2_1",
    "2_2",
    "3_0",
    "3_1",
    "3_2",
    "3_3",
    "4_0",
    "4_1",
    "4_2",
    "4_3",
    "4_4",
    "5_0",
    "5_1",
    "5_2",
    "6_0",
    "6_1",
    "6_2",
]);

export const listCategories: RecordType<string, { name: string, id: ListCategory, subcategories: RecordType<string, { name: string, id: ListCategory }>, order: string[] }> = {
    "0": {
        name: "All Categories",
        id: "0_0",
        subcategories: {},
        order: [],
    },
    "1": {
        name: "Anime",
        id: "1_0",
        subcategories: {
            "1": { name: "AMV", id: "1_1" },
            "2": { name: "English", id: "1_2" },
            "3": { name: "Non-English", id: "1_3" },
            "4": { name: "Raw", id: "1_4" },
        },
        order: ["1", "2", "3", "4"],
    },
    "2": {
        name: "Audio",
        id: "2_0",
        subcategories: {
            "1": { name: "Lossless", id: "2_1" },
            "2": { name: "Lossy", id: "2_2" },
        },
        order: ["1", "2"],
    },
    "3": {
        name: "Literature",
        id: "3_0",
        subcategories: {
            "1": { name: "English", id: "3_1" },
            "2": { name: "Non-English", id: "3_2" },
            "3": { name: "Raw", id: "3_3" },
        },
        order: ["1", "2", "3"],
    },
    "4": {
        name: "Live Action",
        id: "4_0",
        subcategories: {
            "1": { name: "English", id: "4_1" },
            "2": { name: "Idol/PV", id: "4_2" },
            "3": { name: "Non-English", id: "4_3" },
            "4": { name: "Raw", id: "4_4" },
        },
        order: ["1", "2", "3", "4"],
    },
    "5": {
        name: "Pictures",
        id: "5_0",
        subcategories: {
            "1": { name: "Graphics", id: "5_1" },
            "2": { name: "Photos", id: "5_2" },
        },
        order: ["1", "2"],
    },
    "6": {
        name: "Software",
        id: "6_0",
        subcategories: {
            "1": { name: "Apps", id: "6_1" },
            "2": { name: "Games", id: "6_2" },
        },
        order: ["1", "2"],
    },
};

export const listCategoryOrder = [
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
];


export const ListFilterSchema = z.enum([
    "0",
    "1",
    "2",
]);

export function listFilterValueToLabel(value: string): string {
    switch (value) {
        case "0":
            return "No filter";
        case "1":
            return "No remakes";
        case "2":
            return "Trusted only";
        default:
            return "";
    }
}

export const ListSortBySchema = z.enum([
    "id",
    "size",
    "comments",
    "seeders",
    "leechers",
    "downloads",
]);

export const ListSortOrderSchema = z.enum([
    "asc",
    "desc",
]);

export type ListCategory = z.infer<typeof ListCategorySchema>;
export type ListFilter = z.infer<typeof ListFilterSchema>;
export type ListSortBy = z.infer<typeof ListSortBySchema>;
export type ListSortOrder = z.infer<typeof ListSortOrderSchema>;

export const ListItemSchema = z.object({
    id: z.number().int().nonnegative(),
    title: z.string(),
    pub_date: z.string().datetime(),
    description: z.string().nullable().optional(),
    category: z.string(),
    size: z.number().int().nonnegative(),
    seeders: z.number().int().nonnegative(),
    leechers: z.number().int().nonnegative(),
    downloads: z.number().int().nonnegative(),
    comments: z.number().int().nonnegative(),
    trusted: z.boolean(),
    remake: z.boolean(),
});

export const ListRequestSchema = z.object({
    p: z.number().optional().default(1),
    c: ListCategorySchema.optional().default("0_0"),
    s: ListSortBySchema.optional().default("id"),
    o: ListSortOrderSchema.optional().default("desc"),
    f: ListFilterSchema.optional().default("0"),
    q: z.string().optional(),
});

export const ListResponseSchema = z.object({
    items: z.array(ListItemSchema),
});

export type ListItem = z.infer<typeof ListItemSchema>;
export type ListRequest = z.infer<typeof ListRequestSchema>;
export type ListResponse = z.infer<typeof ListResponseSchema>;

export const MirrorResponseSchema = z.object({
    items: z.array(z.object({
        id: z.string(),
        name: z.string(),
    })),
});

export type MirrorResponse = z.infer<typeof MirrorResponseSchema>;

export const MagnetResponseSchema = z.object({
    magnet_link: z.string(),
});

export type MagnetResponse = z.infer<typeof MagnetResponseSchema>;


export const MirrorRouteParamsSchema = z.object({
    mirror: z.string(),
});

export const MirrorViewRouteParamsSchema = z.object({
    mirror: z.string(),
    id: z.number().int().nonnegative(),
});

export const MirrorHealthResponseSchema = z.object({
    mirrors: z.array(
    z.object({
        id: z.string(),
        name: z.string(),
        requests: z.array(z.tuple([z.string().datetime(), z.string(), z.boolean(), z.boolean(), z.number()])),
    })
)});

