import type { Account } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

/*
Invoke is super fast in Tauri so for now we can just call the backend directly

TODO: In the future switch to using a context provider and make categories a singleton
*/
const getAllAccounts = async (): Promise<Account[]> => {
    return (await invoke("get_all_accounts")) as Account[];
}

const getAccountById = async (id: number): Promise<Account | undefined> => {
    const accounts = await getAllAccounts();
    return accounts.find(account => account.id === id);
}

export const accountsApi = {
    getAllAccounts,
    getAccountById,
}