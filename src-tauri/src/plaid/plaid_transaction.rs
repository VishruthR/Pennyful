use chrono::NaiveDate;

use crate::transactions::dollars::Dollars;

#[derive(PartialEq, Debug)]
pub struct PlaidTransaction {
    pub plaid_transaction_id: Option<String>,
    pub name: Option<String>,
    pub merchant_entity_id: Option<String>,
    pub amount: Dollars,
    pub date: NaiveDate,
    pub pending: bool,
    plaid_account_id: String,
    account_id: Option<i64>,
    category_id: Option<i64>,
}

impl PlaidTransaction {
    pub fn plaid_account_id(&self) -> &String {
        &self.plaid_account_id
    }

    #[allow(dead_code)]
    pub fn category_id(&self) -> &Option<i64> {
        &self.category_id
    }

    pub fn account_id(&self) -> &Option<i64> {
        &self.account_id
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        plaid_transaction_id: Option<String>,
        name: Option<String>,
        merchant_entity_id: Option<String>,
        amount: Dollars,
        date: NaiveDate,
        pending: bool,
        plaid_account_id: String,
        account_id: Option<i64>,
        category_id: Option<i64>,
    ) -> Self {
        PlaidTransaction {
            plaid_transaction_id,
            name,
            merchant_entity_id,
            amount,
            date,
            pending,
            plaid_account_id,
            account_id,
            category_id,
        }
    }

    pub fn update_account_id(mut self, account_id: i64) -> Self {
        self.account_id = Some(account_id);
        self
    }
}
