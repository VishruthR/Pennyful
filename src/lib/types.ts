import type { Snippet } from 'svelte';

interface Category {
    id: number,
    name: string,
    color: string,
    secondary_color: string,
    icon: string
}
type CategoryDetails =  Record<string, Category>;

interface CategoryOverview {
    id: number;
    name: string;
    color: string;
    icon?: string;
    budget_cents: number | null;
    spent_cents: number;
}

interface Transaction {
    id: number;
    name: string;
    amount: number;
    date: Date;
    account_id: number;
    category_id: number;
}

type TransactionImport = Omit<Transaction, "id">;

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

interface LinkedInstitution {
  item_id: string;
  institution_name: string;
  institution_id: string | null;
  account_count: number;
}

enum AccountType {
    Savings = "Savings",
    Checkings = "Checkings",
    Credit = "Credit"
}

interface Account {
    id: number;
    plaid_account_id: string | null;
    name: string;
    official_name: string | null;
    bank_id: number;
    bank_name: string;
    account_type: AccountType;
    initial_balance: number;
    available_balance: number;
    current_balance: number;
}

interface DropdownOption {
    value: string;
    content: Snippet;
}

export type { Category, CategoryDetails, CategoryOverview, Transaction, TransactionImport, AccountType, Account, DropdownOption, PlaidAccount, PlaidItem, AccountsGetResponse, LinkedInstitution };
