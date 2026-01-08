use sqlx::{Pool, Sqlite};
use crate::types::{Transaction, Cents};

pub async fn get_transactions(pool: &Pool<Sqlite>, limit: Option<i64>) -> Result<Vec<Transaction>, sqlx::Error> {
    let query = "SELECT id, name, amount_cents, date, account_id, category_id FROM 'transaction' ORDER BY date, id LIMIT $1";

    // Negative value returns all rows
    let lim = limit.unwrap_or(-1);
    let res: Vec<Transaction> = sqlx::query_as(query)
        .bind(lim)
        .fetch_all(pool).await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rust_decimal::dec;
    
    fn get_expected_transactions() -> Vec<Transaction> {
        vec![
            Transaction::new(
                1,
                "TRANSACTION 1".to_owned(),
                Cents(dec!(-5.77)),
                NaiveDate::from_ymd_opt(2025, 12, 15).unwrap(),
                1,
                1
            ),
            Transaction::new(
                2,
                "TRANSACTION 2".to_owned(),
                Cents(dec!(-10.90)),
                NaiveDate::from_ymd_opt(2025, 12, 16).unwrap(),
                1,
                1
            ),
            Transaction::new(
                4,
                "TRANSACTION 4".to_owned(),
                Cents(dec!(-0.70)),
                NaiveDate::from_ymd_opt(2025, 12, 16).unwrap(),
                1,
                1
            ),
            Transaction::new(
                3,
                "TRANSACTION 3".to_owned(),
                Cents(dec!(-1.90)),
                NaiveDate::from_ymd_opt(2025, 12, 17).unwrap(),
                1,
                1
            ),
        ]
    }

    #[sqlx::test(fixtures(path="../fixtures", scripts("transactions")))]
    async fn test_get_transactions_all(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let transactions = get_transactions(&pool, None).await?;
        
        assert_eq!(transactions, get_expected_transactions());
        Ok(())
    }

    #[sqlx::test(fixtures(path="../fixtures", scripts("transactions")))]
    async fn test_get_transactions_limit(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let transactions = get_transactions(&pool, Some(2)).await?;
        
        assert_eq!(transactions, get_expected_transactions()[..2]);
        Ok(())
    }
}