import { invoke } from "@tauri-apps/api/core";

const connectToPlaid = async (): Promise<string> => {
    return (await invoke("connect_to_plaid")) as string;
};

const generateLinkToken = async (): Promise<string> => {
  return (await invoke("generate_link_token")) as string;
}

const generateAccessToken = async (public_token: string): Promise<string> => {
  return (await invoke("generate_access_token", {
    publicToken: public_token
  })) as string;
}

export const plaidApi = {
    connectToPlaid,
    generateLinkToken,
    generateAccessToken
};
