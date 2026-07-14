import type { Account, AccountsGetResponse, PlaidAccount } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

const savePlaidCredentials = async (client_id: string, secret: string): Promise<string> => {
  return (await invoke("save_plaid_credentials", {
    clientId: client_id,
    secret: secret
  })) as string;
}

const generateLinkToken = async (): Promise<string> => {
  return (await invoke("generate_link_token")) as string;
}

const generateAccessTokenFromHostedLink = async (): Promise<string> => {
  return (await invoke("generate_access_token_from_hosted_link")) as string;
}

const getAccountsOfItem = async (item_id: string): Promise<Account[]> => {
  return (await invoke("get_accounts_of_item", {
    itemId: item_id
  })) as Account[];
}

const getAccountsOfItemFromPlaid = async (item_id: string): Promise<AccountsGetResponse> => {
  return (await invoke("get_accounts_of_item_from_plaid", {
    itemId: item_id
  })) as AccountsGetResponse;
}

const fetchItemAndAccounts = async (item_id: string): Promise<string> => {
  return (await invoke("fetch_item_and_accounts", {
    itemId: item_id
  })) as string;
}

const addNewPlaidAccounts = async (accounts: PlaidAccount[], item_id: string): Promise<number> => {
  return (await invoke("add_new_plaid_accounts", {
    newAccounts: accounts,
    itemId: item_id
  }))
}

const syncTransactions = async (item_id: string): Promise<number> => {
  return (await invoke("sync_transactions", {
    itemId: item_id,
    daysRequested: 30,
  })) as number;
}

export const plaidApi = {
    savePlaidCredentials,
    generateLinkToken,
    generateAccessTokenFromHostedLink,
    fetchItemAndAccounts,
    syncTransactions,
    getAccountsOfItem,
    getAccountsOfItemFromPlaid,
    addNewPlaidAccounts
};
