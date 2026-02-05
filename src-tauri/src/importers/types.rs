use std::fmt;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct TransactionImport {
    #[serde(deserialize_with = "de_date_from_str")]
    pub date: NaiveDate,
    pub name: String,
    #[serde(deserialize_with = "de_decimal_from_money_str")]
    pub amount: Decimal,
}

impl fmt::Display for TransactionImport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction: {}, {}, {}", self.date, self.name, self.amount)
    }
}

fn de_date_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%m/%d/%Y").map_err(serde::de::Error::custom)
}

// Custom deserializer since banks often include thousands seperators
fn de_decimal_from_money_str<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let clean = s.replace(['"', ','], "").trim().to_string();
    Decimal::from_str(&clean).map_err(serde::de::Error::custom)
}