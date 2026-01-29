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

enum AccountType {
    Savings = "Savings",
    Checkings = "Checkings",
}

interface BankAccount {
    id: number;
    name: string;
    bank_id: number;
    bank_name: string;
    account_type: AccountType;
    initial_balance: number;
    current_balance: number;
}

export type { Category, CategoryDetails, FullTransactionInfo, BankAccount };

