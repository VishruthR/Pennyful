import { openUrl } from "@tauri-apps/plugin-opener";

export const PLAID_URL = "https://plaid.com";
export const PLAID_TRIAL_URL = "https://dashboard.plaid.com/signup";
export const GITHUB_REPO_URL = "https://github.com/pennyful/pennyful";

export const openExternal = (url: string) => (event: MouseEvent) => {
  event.preventDefault();
  openUrl(url);
};

