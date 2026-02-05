import type { BankAccount } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

/*
Invoke is super fast in Tauri so for now we can just call the backend directly

TODO: In the future switch to using a context provider and make categories a singleton
*/
const getAllAccounts = async (): Promise<BankAccount[]> => {
    return (await invoke("get_all_accounts")) as BankAccount[];
}

export const accountsApi = {
    getAllAccounts,
}