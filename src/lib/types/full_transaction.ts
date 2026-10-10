import type { Transaction } from "$lib/types/transaction";

interface FullTransaction {
  transaction: Transaction;
  category_name: string;
  category_color: string;
  category_icon: string | null;
  account_name: string;
  bank_institution_id: string | null;
}

export type { FullTransaction };
