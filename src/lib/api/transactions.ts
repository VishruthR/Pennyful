import { invoke } from "@tauri-apps/api/core";

const syncTransactions = async (): Promise<void> => {
    return (await invoke("sync_transactions")) ;
}

export const transactionsApi = {
  syncTransactions,
}
