enum AccountType {
  Savings = "Savings",
  Checkings = "Checkings",
  Credit = "Credit",
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

export type { Account };
