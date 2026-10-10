import type { FullTransaction } from "$lib/types/full_transaction";
import type { PaginedSortedTransactionsResponse } from "$lib/types/paginated_sorted_transactions_response";
import { invoke } from "@tauri-apps/api/core";

const syncTransactions = async (): Promise<void> => {
  return await invoke("sync_transactions");
};

const getPaginatedSortedTransactions = async (
  page: number,
  page_size: number,
  sort_col: string | null,
  sort_dir: string | null,
): Promise<PaginedSortedTransactionsResponse> => {
  return (await invoke("get_paginated_sorted_transactions", {
    page: page,
    pageSize: page_size,
    sortCol: sort_col,
    sortDir: sort_dir,
  })) as PaginedSortedTransactionsResponse;
};

const getTransactionsByCategory = async (
  categoryName: string,
): Promise<FullTransaction[]> => {
  return (await invoke("get_transactions_by_category", {
    categoryName,
  })) as FullTransaction[];
};

const getNumTransactionsByCategory = async (
  categoryName: string,
): Promise<number> => {
  return (await invoke("get_num_transactions_by_category", {
    categoryName,
  })) as number;
};

const updateTransactionCategory = async (
  transactionId: number,
  categoryId: number,
): Promise<void> => {
  await invoke("update_transaction_category", { transactionId, categoryId });
};

const getSpendingAndIncomeByDateRange = async (
  startDate: string,
  endDate: string,
): Promise<[number, number]> => {
  return (await invoke("get_spending_and_income_by_date_range", {
    startDate,
    endDate,
  })) as [number, number];
};

export const transactionsApi = {
  syncTransactions,
  getPaginatedSortedTransactions,
  updateTransactionCategory,
  getTransactionsByCategory,
  getNumTransactionsByCategory,
  getSpendingAndIncomeByDateRange,
};
