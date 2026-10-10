import type { FullTransaction } from "./full_transaction";

interface PaginedSortedTransactionsResponse {
  transactions: FullTransaction[];
  curr_page: number;
  next_page: number | null;
  prev_page: number | null;
  num_pages: number;
  num_transactions: number;
}

export type { PaginedSortedTransactionsResponse };
