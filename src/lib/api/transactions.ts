import type { TransactionWithAccount } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

const syncTransactions = async (): Promise<void> => {
    return (await invoke("sync_transactions")) ;
}

const getPaginatedSortedTransactions = async (
  page: number,
  page_size: number,
  sort_col: string | null,
  sort_dir: string | null
): Promise<TransactionWithAccount[]> => {
  return (await invoke("get_paginated_sorted_transactions", {
    page: page,
    pageSize: page_size,
    sortCol: sort_col,
    sortDir: sort_dir
  })) as TransactionWithAccount[];
}

export const transactionsApi = {
  syncTransactions,
  getPaginatedSortedTransactions
}
