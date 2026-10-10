#![allow(dead_code)]
use crate::transactions::dollars::Dollars;

#[derive(PartialEq, Debug)]
pub struct Balances {
    available: Dollars,
    current: Dollars,
    limit: Dollars,
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
