import type { Snippet } from 'svelte';

interface Category {
    id: number,
    name: string,
    color: string,
    secondary_color: string,
    icon: string
}
type CategoryDetails =  Map<string, Category>;

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

interface DropdownOption {
    value: string;
    content: Snippet;
}

export type { Category, CategoryDetails, FullTransactionInfo, DropdownOption };

