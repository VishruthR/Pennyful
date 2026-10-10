import type { PlaidAccount } from "./plaid_account";
import type { PlaidItem } from "./plaid_item";

interface AccountsGetResponse {
  accounts: PlaidAccount[];
  item: PlaidItem;
}

export type { AccountsGetResponse };
