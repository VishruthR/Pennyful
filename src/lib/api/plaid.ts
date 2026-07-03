import { invoke } from "@tauri-apps/api/core";

const connectToPlaid = async (): Promise<string> => {
    return (await invoke("connect_to_plaid")) as string;
};

export const plaidApi = {
    connectToPlaid,
};
