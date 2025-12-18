use std::fmt;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct Transaction {
    #[serde(deserialize_with = "de_date_from_str")]
    pub date: NaiveDate,
    pub description: String,
    #[serde(deserialize_with = "de_float_from_money_str")]
    pub amount: f64,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction: {}, {}, {}", self.date, self.description, self.amount)
    }
}

fn de_date_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%m/%d/%Y").map_err(serde::de::Error::custom)
}

// Custom deserializer since banks often include thousands seperators
fn de_float_from_money_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let clean = s.replace(',', "").replace('\"', "").trim().to_string();
    clean.parse::<f64>().map_err(serde::de::Error::custom)
}