use std::fmt;
use chrono::NaiveDate;
use rust_decimal::{prelude::{FromPrimitive, ToPrimitive}, Decimal};
use sqlx::{Sqlite, decode::Decode, encode::{Encode, IsNull}, Type};

// Custom type to enable automatic encoding/decoding for sqlx
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, serde::Serialize)]
pub struct Cents (pub Decimal);

impl Type<Sqlite> for Cents {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
}

impl fmt::Display for Cents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'q> Encode<'q, Sqlite> for Cents {
    fn encode_by_ref(
        &self, 
        args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>
    ) -> Result<IsNull, Box<dyn std::error::Error + Send + Sync>> {
        // Multiply by 100 and convert to i64
        let cents = (self.0 * Decimal::from(100))
            .to_i64()
            .expect("Decimal overflow when converting to cents");
        
        <i64 as Encode<Sqlite>>::encode(cents, args)
    }
}

impl<'r> Decode<'r, Sqlite> for Cents {
    fn decode(
        value: sqlx::sqlite::SqliteValueRef<'r>
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Convert back to Decimal and divide by 100
        let cents = <i64 as Decode<Sqlite>>::decode(value)?;
        let decimal = Decimal::from(cents) / Decimal::from(100);

        Ok(Cents(decimal))
    }
}

impl Cents {
    pub fn from_dollars_f64(dollars: f64) -> Option<Self> {
        Decimal::from_f64(dollars).map(|d| Cents(d.round_dp(2)))
    }
}


#[derive(sqlx::FromRow, PartialEq, Debug)]
pub struct Transaction {
    id: i64,
    plaid_transaction_id: Option<String>,
    pub name: String,
    merchant_entity_id: Option<String>,
    #[sqlx(rename = "amount_cents")]
    pub amount: Cents,
    pub date: NaiveDate,
    pub pending: bool,
    pub deleted_at: Option<NaiveDate>,
    plaid_account_id: Option<String>,
    account_id: i64,
    category_id: i64,
}

impl Transaction {
    pub fn id(&self) -> &i64 {
        &self.id
    }

    pub fn plaid_transaction_id(&self) -> &Option<String> {
        &self.plaid_transaction_id
    }

    pub fn plaid_account_id(&self) -> &Option<String> {
        &self.plaid_account_id
    }

    pub fn account_id(&self) -> &i64 {
        &self.account_id
    }

    pub fn category_id(&self) -> &i64 {
        &self.category_id
    }

    pub fn new(
        id: i64,
        name: String,
        amount: Cents,
        date: NaiveDate,
        account_id: i64,
        category_id: i64
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
            plaid_account_id: None,
            account_id,
            category_id,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction: {} {} {} {}, {:?}, {}",
            self.id, self.date, self.name, self.amount, self.category_id, self.account_id)
    }
}

/// A transaction not yet persisted. Has no id, since that is assigned by the DB
/// on insert. Used to hold data returned from Plaid before it is written.
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

#[derive(sqlx::FromRow, Eq, PartialEq, Debug, Clone, serde::Serialize)]
pub struct Category {
    id: i64,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, serde::Serialize)]
#[sqlx(rename_all = "UPPERCASE")]
pub enum AccountType {
    Savings,
    Checkings,
    Credit,
}

#[derive(sqlx::FromRow, PartialEq, Debug, Clone, serde::Serialize)]
pub struct Account {
    pub id: i64,
    pub plaid_account_id: Option<String>,
    pub name: String,
    pub bank_id: i64,
    pub account_type: AccountType,
    #[sqlx(rename = "initial_balance_cents")]
    pub initial_balance: Cents,
    #[sqlx(rename = "current_balance_cents")]
    pub current_balance: Cents,
}

impl Account {
    pub fn new(
        id: i64,
        name: String,
        bank_id: i64,
        account_type: AccountType,
        initial_balance: Cents,
        current_balance: Cents,
    ) -> Self {
        Account {
            id,
            plaid_account_id: None,
            name,
            bank_id,
            account_type,
            initial_balance,
            current_balance,
        }
    }
}

/// Account with bank information joined from the bank table
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
    pub fn new(
        id: i64,
        name: String,
        bank_id: i64,
        bank_name: String,
        account_type: AccountType,
        initial_balance: Cents,
        current_balance: Cents,
    ) -> Self {
        FullAccount {
            account: Account::new(id, name, bank_id, account_type, initial_balance, current_balance),
            bank_name,
        }
    }
}

#[derive(sqlx::FromRow, PartialEq, Debug)]
pub struct PlaidItem {
    item_id: String,
    access_token: String,
    cursor: Option<String>
}

impl PlaidItem {
    pub fn item_id(&self) -> &String {
        &self.item_id
    }

    pub fn access_token(&self) -> &String {
        &self.access_token
    }

    pub fn cursor(&self) -> &Option<String> {
        &self.cursor
    }

    pub fn new(
        item_id: String,
        access_token: String,
        cursor: Option<String>
    ) -> Self {
        PlaidItem {
            item_id, access_token, cursor
        }
    }
}

impl fmt::Display for PlaidItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PlaidItem: {} {}", 
            self.item_id, self.cursor.clone().unwrap_or("No cursor".to_string()))
    }
}
