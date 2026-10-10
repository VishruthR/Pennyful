use crate::accounts::account_type::AccountType;
use crate::transactions::dollars::Dollars;

#[derive(sqlx::FromRow, PartialEq, Debug, Clone, serde::Serialize)]
pub struct Account {
    pub id: i64,
    pub plaid_account_id: Option<String>,
    pub name: String,
    pub official_name: Option<String>,
    pub bank_id: i64,
    pub account_type: AccountType,
    #[sqlx(rename = "initial_balance_cents")]
    pub initial_balance: Dollars,
    #[sqlx(rename = "available_balance_cents")]
    pub available_balance: Dollars,
    #[sqlx(rename = "current_balance_cents")]
    pub current_balance: Dollars,
}

impl Account {
    pub fn new(
        id: i64,
        plaid_account_id: Option<String>,
        name: String,
        official_name: Option<String>,
        bank_id: i64,
        account_type: AccountType,
        initial_balance: Dollars,
        available_balance: Dollars,
        current_balance: Dollars,
    ) -> Self {
        Account {
            id,
            plaid_account_id,
            name,
            official_name,
            bank_id,
            account_type,
            initial_balance,
            available_balance,
            current_balance,
        }
    }
}
