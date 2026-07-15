use chrono::NaiveDate;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use sqlx::{
    decode::Decode,
    encode::{Encode, IsNull},
    Sqlite, Type,
};
use std::fmt;

// Custom type to enable automatic encoding/decoding for sqlx
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, serde::Serialize)]
pub struct Cents(pub Decimal);

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
        args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>,
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
        value: sqlx::sqlite::SqliteValueRef<'r>,
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
    pub official_name: Option<String>,
    pub bank_id: i64,
    pub account_type: AccountType,
    #[sqlx(rename = "initial_balance_cents")]
    pub initial_balance: Cents,
    #[sqlx(rename = "available_balance_cents")]
    pub available_balance: Cents,
    #[sqlx(rename = "current_balance_cents")]
    pub current_balance: Cents,
}

impl Account {
    pub fn new(
        id: i64,
        plaid_account_id: Option<String>,
        name: String,
        official_name: Option<String>,
        bank_id: i64,
        account_type: AccountType,
        initial_balance: Cents,
        available_balance: Cents,
        current_balance: Cents,
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

#[derive(sqlx::FromRow, PartialEq, Debug)]
pub struct PlaidItem {
    item_id: String,
    access_token: String,
    cursor: Option<String>,
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

    pub fn new(item_id: String, access_token: String, cursor: Option<String>) -> Self {
        PlaidItem {
            item_id,
            access_token,
            cursor,
        }
    }
}

impl fmt::Display for PlaidItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PlaidItem: {} {}",
            self.item_id,
            self.cursor.clone().unwrap_or("No cursor".to_string())
        )
    }
}

#[derive(sqlx::FromRow, PartialEq, Debug, serde::Serialize)]
pub struct Bank {
    id: i64,
    plaid_item_id: Option<String>,
    plaid_institution_id: Option<String>,
    bank_name: String,
}

/// A previously-linked institution surfaced to the frontend so the user can add
/// more of its accounts without re-linking through Plaid Link.
#[derive(sqlx::FromRow, serde::Serialize, Debug, PartialEq)]
pub struct LinkedInstitution {
    pub item_id: String,
    pub institution_name: String,
    pub account_count: i64,
}

impl Bank {
    pub fn new(
        id: i64,
        plaid_item_id: Option<String>,
        plaid_institution_id: Option<String>,
        bank_name: String,
    ) -> Self {
        Bank {
            id,
            plaid_item_id,
            plaid_institution_id,
            bank_name,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn plaid_item_id(&self) -> &Option<String> {
        &self.plaid_item_id
    }

    pub fn plaid_institution_id(&self) -> &Option<String> {
        &self.plaid_institution_id
    }

    pub fn bank_name(&self) -> &String {
        &self.bank_name
    }
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bank: {} {} {} {}",
            self.id,
            self.plaid_item_id
                .clone()
                .unwrap_or("no_item_id".to_string()),
            self.plaid_institution_id
                .clone()
                .unwrap_or("no_institution_id".to_string()),
            self.bank_name
        )
    }
}
