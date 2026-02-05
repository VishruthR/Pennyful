use std::fmt;
use chrono::NaiveDate;
use rust_decimal::{prelude::ToPrimitive, Decimal};
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


#[derive(sqlx::FromRow, PartialEq, Debug)]
pub struct Transaction {
    id: u64,
    pub name: String,
    #[sqlx(rename = "amount_cents")]
    pub amount: Cents,
    pub date: NaiveDate,
    account_id: u64,
    category_id: u64,
}

impl Transaction {
    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn account_id(&self) -> &u64 {
        &self.account_id
    }

    pub fn category_id(&self) -> &u64 {
        &self.category_id
    }

    pub fn new(
        id: u64,
        name: String,
        amount: Cents,
        date: NaiveDate,
        account_id: u64,
        category_id: u64
    ) -> Self {
        Transaction {
            id, name, amount, date, account_id, category_id
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction: {} {} {} {}, {:?}, {}", 
            self.id, self.date, self.name, self.amount, self.category_id, self.account_id)
    }
}

#[derive(sqlx::FromRow, Eq, PartialEq, Debug, Clone, serde::Serialize)]
pub struct Category {
    id: u64,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, serde::Serialize)]
#[sqlx(rename_all = "UPPERCASE")]
pub enum AccountType {
    Savings,
    Checkings,
}

#[derive(sqlx::FromRow, PartialEq, Debug, Clone, serde::Serialize)]
pub struct Account {
    id: u64,
    pub name: String,
    bank_id: u64,
    pub account_type: AccountType,
    #[sqlx(rename = "initial_balance_cents")]
    pub initial_balance: Cents,
    #[sqlx(rename = "current_balance_cents")]
    pub current_balance: Cents,
}

impl Account {
    pub fn new(
        id: u64,
        name: String,
        bank_id: u64,
        account_type: AccountType,
        initial_balance: Cents,
        current_balance: Cents,
    ) -> Self {
        Account {
            id,
            name,
            bank_id,
            account_type,
            initial_balance,
            current_balance,
        }
    }
}

/// Account with bank information joined from the bank table
#[derive(sqlx::FromRow, PartialEq, Debug, Clone, serde::Serialize)]
pub struct FullAccount {
    pub id: u64,
    pub name: String,
    pub bank_id: u64,
    pub bank_name: String,
    pub account_type: AccountType,
    #[sqlx(rename = "initial_balance_cents")]
    pub initial_balance: Cents,
    #[sqlx(rename = "current_balance_cents")]
    pub current_balance: Cents,
}

impl FullAccount {
    pub fn new(
        id: u64,
        name: String,
        bank_id: u64,
        bank_name: String,
        account_type: AccountType,
        initial_balance: Cents,
        current_balance: Cents,
    ) -> Self {
        FullAccount {
            id,
            name,
            bank_id,
            bank_name,
            account_type,
            initial_balance,
            current_balance,
        }
    }
}