import { invoke } from "@tauri-apps/api/core";

const connectToPlaid = async (): Promise<string> => {
    return (await invoke("connect_to_plaid")) as string;
};

const generateLinkToken = async (): Promise<string> => {
  return (await invoke("generate_link_token")) as string;
}

const generateAccessTokenFromHostedLink = async (): Promise<string> => {
  return (await invoke("generate_access_token_from_hosted_link")) as string;
}

export const plaidApi = {
    connectToPlaid,
    generateLinkToken,
    generateAccessTokenFromHostedLink
};
