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
#[derive(
    Debug, Clone, Copy, Default, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
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

    pub fn from_i32(dollars: i32) -> Option<Self> {
        Decimal::from_i32(dollars).map(|d| Cents(d))
    }

    pub fn from_i32_or_throw(dollars: i32) -> Self {
        Self::from_i32(dollars).expect("Coudln't convert i32 value to Cents")
    }
}
