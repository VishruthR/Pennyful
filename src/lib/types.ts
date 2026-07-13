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

type TransactionImport = Omit<Transaction, "id">;

enum AccountType {
    Savings = "Savings",
    Checkings = "Checkings",
}

interface PlaidAccount {
  account_id: string;
  balances: {
    available: number;
    current: number;
    limit: number;
  };
  mask: string | null;
  name: string;
  official_name: string | null;
  type: string;
  subtype: string | null;
}

interface PlaidItem {
  item_id: string;
  institution_id: string | null;
  institution_name: string | null;
}

interface AccountsGetResponse {
  accounts: PlaidAccount[];
  item: PlaidItem;
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

interface DropdownOption {
    value: string;
    content: Snippet;
}

export type { Category, CategoryDetails, Transaction, TransactionImport, AccountType, Account, DropdownOption, PlaidAccount, PlaidItem, AccountsGetResponse };
