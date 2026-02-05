import type { TransactionImport } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";
import { categoriesApi } from "./categories";

const importTransactions = async (filePath: string, bankName: string, accountId: number): Promise<TransactionImport[]> => {
    const uncategorizedCategory = await categoriesApi.getCategoryByName("Uncategorized");
    const transactions =  await invoke("import_transactions", {
        filePath: filePath,
        bankName: bankName,
    }) as Omit<TransactionImport, "account_id" | "category_id">[];

    const populatedTransactions = transactions.map(transaction => ({ 
        ...transaction, 
        date: new Date(transaction.date), 
        account_id: accountId,
        category_id: uncategorizedCategory?.id ?? 0
    }));

    return populatedTransactions;
};

export { importTransactions };