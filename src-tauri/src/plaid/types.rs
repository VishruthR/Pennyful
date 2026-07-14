use chrono::NaiveDate;

use crate::types::Cents;

#[derive(PartialEq, Debug)]
pub struct Balances {
    available: Cents,
    current: Cents,
    limit: Cents,
}

#[derive(PartialEq, Debug)]
pub struct PlaidAccount {
    account_id: String,
    pub balances: Balances,
    pub mask: Option<String>,
    pub name: String,
    pub official_name: Option<String>,
    pub type_: String,
    pub subtype: Option<String>,
}

impl PlaidAccount {
    pub fn account_id(&self) -> &String {
        &self.account_id
    }

    pub fn new(
        account_id: String,
        balances: Balances,
        mask: Option<String>,
        name: String,
        official_name: Option<String>,
        type_: String,
        subtype: Option<String>,
    ) -> Self {
        PlaidAccount {
            account_id,
            balances,
            mask,
            name,
            official_name,
            type_,
            subtype,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct PlaidTransaction {
    pub plaid_transaction_id: Option<String>,
    pub name: Option<String>,
    pub merchant_entity_id: Option<String>,
    pub amount: Cents,
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

    pub fn category_id(&self) -> &Option<i64> {
        &self.category_id
    }

    pub fn account_id(&self) -> &Option<i64> {
        &self.account_id
    }

    pub fn new(
        plaid_transaction_id: Option<String>,
        name: Option<String>,
        merchant_entity_id: Option<String>,
        amount: Cents,
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
