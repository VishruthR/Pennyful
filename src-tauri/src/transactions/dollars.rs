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

// Custom type to represent financial quantities. `rust_decimal` provides the foundation.
// However, sqlx doesn't support a "decimal" type. Therefore, we convert to an int and multiply
// the quantity by 100 when writing to sqlx.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct Dollars(pub Decimal);

impl Type<Sqlite> for Dollars {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
}

impl fmt::Display for Dollars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'q> Encode<'q, Sqlite> for Dollars {
    fn encode_by_ref(
        &self,
        args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>,
    ) -> Result<IsNull, Box<dyn std::error::Error + Send + Sync>> {
        // Multiply by 100 and convert to i64
        let dollars = (self.0 * Decimal::from(100))
            .to_i64()
            .expect("Decimal overflow when converting to dollars");

        <i64 as Encode<Sqlite>>::encode(dollars, args)
    }
}

impl<'r> Decode<'r, Sqlite> for Dollars {
    fn decode(
        value: sqlx::sqlite::SqliteValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Convert back to Decimal and divide by 100
        let dollars = <i64 as Decode<Sqlite>>::decode(value)?;
        let decimal = Decimal::from(dollars) / Decimal::from(100);

        Ok(Dollars(decimal))
    }
}

impl Dollars {
    pub fn from_dollars_f64(dollars: f64) -> Option<Self> {
        Decimal::from_f64(dollars).map(|d| Dollars(d.round_dp(2)))
    }

    pub fn from_i32(dollars: i32) -> Option<Self> {
        Decimal::from_i32(dollars).map(|d| Dollars(d))
    }

    pub fn from_i32_or_throw(dollars: i32) -> Self {
        Self::from_i32(dollars).expect("Coudln't convert i32 value to Dollars")
    }
}
