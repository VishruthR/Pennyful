interface Transaction {
  id: number;
  name: string;
  amount: number;
  // Rust backend treats Date as day, Date class in JS represents an instant
  // To avoid mismatches, we just treat it as a string
  // Expect dates in YYYY-MM-DD format
  date: string;
  account_id: number;
  category_id: number;
}

export type { Transaction };
