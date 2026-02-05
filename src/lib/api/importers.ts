import type { TransactionImport } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";
import { getCategoryByName } from "./categories";

const importTransactions = async (filePath: string, bankName: string, accountId: number): Promise<TransactionImport[]> => {
    const transactions =  await invoke("import_transactions", {
        filePath: filePath,
        bankName: bankName,
    }) as TransactionImport[];
    const uncategorizedCategory = await getCategoryByName("Uncategorized");

    const populatedTransactions = transactions.map(transaction => ({ 
        ...transaction, 
        date: new Date(transaction.date), 
        account_id: accountId,
        category_id: transaction.category_id ?? uncategorizedCategory?.id ?? 0
    }));

    return populatedTransactions;
};

export { importTransactions };