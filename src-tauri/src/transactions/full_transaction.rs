use crate::transactions::transaction::Transaction;
use std::ops::Deref;

#[derive(sqlx::FromRow, PartialEq, Debug, serde::Serialize)]
pub struct FullTransaction {
    pub transaction: Transaction,
    pub category_name: String,
    pub category_color: String,
    pub category_icon: Option<String>,
    account_name: String,
    bank_institution_id: Option<String>,
}

impl Deref for FullTransaction {
    type Target = Transaction;

    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for FullTransaction {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        Ok(FullTransaction {
            transaction: Transaction::from_row(row)?,
            category_name: row.try_get("category_name")?,
            category_color: row.try_get("category_color")?,
            category_icon: row.try_get("category_icon")?,
            account_name: row.try_get("account_name")?,
            bank_institution_id: row.try_get("bank_institution_id")?,
        })
    }
}
