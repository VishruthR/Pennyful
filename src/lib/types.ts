interface Category {
    id: number,
    name: string,
    color: string,
    secondary_color: string,
    icon: string
}
type CategoryDetails =  Map<String, Category>;

interface FullTransactionInfo {
    id: number;
    name: string;
    amount: number;
    date: Date;
    account: {
        id: number;
        name: string;
    };
    category: {
        id: number;
        name: string;
        color: string;
        icon?: string;
    };
}

export type { Category, CategoryDetails, FullTransactionInfo };

