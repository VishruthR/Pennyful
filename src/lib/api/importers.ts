import type { TransactionImport } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

const importTransactions = async (filePath: string, bankName: string, accountId: number): Promise<TransactionImport[]> => {
    const transactions =  await invoke("import_transactions", {
        filePath: filePath,
        bankName: bankName,
    }) as TransactionImport[];

    const populatedTransactions = transactions.map(transaction => ({ ...transaction, date: new Date(transaction.date), account_id: accountId }));

    return populatedTransactions;
};

export { importTransactions };