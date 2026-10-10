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

export type { PlaidAccount };
