use crate::accounts::{account::Account, account_type::AccountType};
use crate::transactions::dollars::Dollars;

// Account with bank information joined from the bank table
#[derive(PartialEq, Debug, Clone, serde::Serialize)]
pub struct FullAccount {
    #[serde(flatten)]
    pub account: Account,
    pub bank_name: String,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for FullAccount {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        Ok(FullAccount {
            account: Account::from_row(row)?,
            bank_name: row.try_get("bank_name")?,
        })
    }
}

impl FullAccount {
    #[allow(dead_code)]
    pub fn new(
        id: i64,
        name: String,
        bank_id: i64,
        bank_name: String,
        account_type: AccountType,
        initial_balance: Dollars,
        current_balance: Dollars,
    ) -> Self {
        FullAccount {
            account: Account::new(
                id,
                None,
                name,
                None,
                bank_id,
                account_type,
                initial_balance,
                current_balance,
                current_balance,
            ),
            bank_name,
        }
    }
}
