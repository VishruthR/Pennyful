use crate::transactions::dollars::Dollars;
use chrono::NaiveDate;
use std::fmt;

#[derive(sqlx::FromRow, PartialEq, Debug, serde::Serialize)]
pub struct Transaction {
    id: i64,
    plaid_transaction_id: Option<String>,
    pub name: String,
    merchant_entity_id: Option<String>,
    #[sqlx(rename = "amount_cents")]
    pub amount: Dollars,
    pub date: NaiveDate,
    pub pending: bool,
    pub deleted_at: Option<NaiveDate>,
    account_id: i64,
    category_id: i64,
}

impl Transaction {
    #[allow(dead_code)]
    pub fn id(&self) -> &i64 {
        &self.id
    }

    #[allow(dead_code)]
    pub fn plaid_transaction_id(&self) -> &Option<String> {
        &self.plaid_transaction_id
    }

    #[allow(dead_code)]
    pub fn account_id(&self) -> &i64 {
        &self.account_id
    }

    #[allow(dead_code)]
    pub fn category_id(&self) -> &i64 {
        &self.category_id
    }

    #[allow(dead_code)]
    pub fn new(
        id: i64,
        name: String,
        amount: Dollars,
        date: NaiveDate,
        account_id: i64,
        category_id: i64,
    ) -> Self {
        Transaction {
            id,
            plaid_transaction_id: None,
            name,
            merchant_entity_id: None,
            amount,
            date,
            pending: false,
            deleted_at: None,
            account_id,
            category_id,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Transaction: {} {} {} {}, {:?}, {}",
            self.id, self.date, self.name, self.amount, self.category_id, self.account_id
        )
    }
}
