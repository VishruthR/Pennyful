import type { Snippet } from 'svelte';

interface Category {
    id: number,
    name: string,
    color: string,
    secondary_color: string,
    icon: string
}
type CategoryDetails =  Record<string, Category>;

interface Transaction {
    id: number;
    name: string;
    amount: number;
    date: Date;
    account_id: number;
    category_id: number;
}

<<<<<<< HEAD
type TransactionImport = Omit<Transaction, "id">;

enum AccountType {
    Savings = "Savings",
    Checkings = "Checkings",
}

interface Account {
    id: number;
    name: string;
    bank_id: number;
    bank_name: string;
    account_type: AccountType;
    initial_balance: number;
    current_balance: number;
}

export type { Category, CategoryDetails, Transaction, Account, TransactionImport };
=======
interface DropdownOption {
    value: string;
    content: Snippet;
}

export type { Category, CategoryDetails, FullTransactionInfo, DropdownOption };
>>>>>>> 690a756 (Start on dropdown component, needs review)

