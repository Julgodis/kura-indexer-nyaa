import { TableCell } from "./table";

function categorySplit(category_id: string): { mainCategory: string, subcategory: string } {
    const categoryMap: Record<string, string> = {
        '0': 'All',
        '1': 'Anime',
        '2': 'Music',
        '3': 'Literature',
        '4': 'Live Action',
        '5': 'Pictures',
        '6': 'Software',
    };

    const subcategories: Record<string, string> = {
        '1_1': 'AMV',
        '1_2': 'English',
        '1_3': 'Non-English',
        '1_4': 'Raw',
        '2_1': 'Lossless',
        '2_2': 'Lossy',
        '3_1': 'English',
        '3_2': 'Non-English',
        '3_3': 'Raw',
        '4_1': 'English',
        '4_2': 'Idol/PV',
        '4_3': 'Non-English',
        '4_4': 'Raw',
        '5_1': 'Graphics',
        '5_2': 'Photos',
        '6_1': 'Apps',
        '6_2': 'Games',
    };

    const mainCategory = categoryMap[category_id.split('_')[0]];
    const subcategory = subcategories[category_id];

    return { mainCategory, subcategory };
}

function adultCategorySplit(category_id: string): { mainCategory: string, subcategory: string } {
    const categoryMap: Record<string, string> = {
        '0': 'All',
        '1': 'Art',
        '2': 'Real Life',
    };

    const subcategories: Record<string, string> = {
        '1_1': 'Anime',
        '1_2': 'Doujinshi',
        '1_3': 'Games',
        '1_4': 'Manga',
        '1_5': 'Pictures',
        '2_1': 'Pictures',
        '2_2': 'Videos',
    };

    const mainCategory = categoryMap[category_id.split('_')[0]];
    const subcategory = subcategories[category_id];

    return { mainCategory, subcategory };
}


export function CategoryCell({ mirrorType, category }: { mirrorType: "normal" | "adult", category: string }) {
    const { mainCategory, subcategory } = mirrorType === "normal" ? categorySplit(category) : adultCategorySplit(category);

    if (!category) {
        return <TableCell><span className="text-xs text-muted-foreground">Unknown Category</span></TableCell>;
    } else if (!subcategory) {
        return <TableCell><span>{mainCategory}</span></TableCell>;
    } else {
        return <TableCell><div className="flex flex-col">
            <span className='text-xs text-muted-foreground'>{mainCategory}</span>
            <span className='text-xs'>{subcategory}</span>
        </div></TableCell>
    }
}