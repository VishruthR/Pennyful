use sqlx::{Pool, QueryBuilder, Sqlite};
use futures::future::join_all;
use crate::types::{PlaidTransaction, Transaction};
use ::plaid::model::RemovedTransaction;

pub async fn get_transactions(pool: &Pool<Sqlite>, limit: Option<i64>) -> Result<Vec<Transaction>, sqlx::Error> {
    let query = "SELECT id, name, amount_cents, date, account_id, category_id FROM 'transaction' ORDER BY date, id LIMIT $1";

    // Negative value returns all rows
    let lim = limit.unwrap_or(-1);
    let res: Vec<Transaction> = sqlx::query_as(query)
        .bind(lim)
        .fetch_all(pool).await?;

    Ok(res)
}

pub async fn add_plaid_transactions(pool: &Pool<Sqlite>, new_transactions: Vec<PlaidTransaction>) -> Result<u64, sqlx::Error> {
    if new_transactions.is_empty() {
        return Ok(0);
    } 
    let num_transactions = new_transactions.len() as u64;

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "INSERT INTO 'transaction' (plaid_transaction_id, name, merchant_entity_id, amount_cents, date, pending, plaid_account_id, account_id, category_id) "
    );

    query_builder.push_values(new_transactions, |mut b, t| {
        // Copy the values reached through getters first, which ends the borrow of
        // `t` so the owned fields below can be moved into the query.
        let plaid_account_id = t.plaid_account_id().clone();
        let account_id = *t.account_id();

        b.push_bind(t.plaid_transaction_id)
         .push_bind(t.name)
         .push_bind(t.merchant_entity_id)
         .push_bind(t.amount)
         .push_bind(t.date)
         .push_bind(t.pending)
         .push_bind(plaid_account_id)
         .push_bind(account_id)
         .push_bind(0); // "Uncategorized" category
    });
    query_builder.push(" ON CONFLICT(plaid_transaction_id) WHERE plaid_transaction_id IS NOT NULL DO NOTHING");

    let query = query_builder.build();
    let res = query.execute(pool).await?;

    Ok(num_transactions - res.rows_affected())
}

pub async fn modify_plaid_transactions(pool: &Pool<Sqlite>, modified_transactions: Vec<PlaidTransaction>) -> Result<(), sqlx::Error> {
    let query = r#"
        UPDATE 'transaction'
        SET amount_cents=$1,
            date=$2,
            name=$3,
            merchant_entity_id=$4
        WHERE plaid_transaction_id=$4 AND plaid_account_id=$5
    "#;

    let modified_queries = modified_transactions
        .into_iter()
        .map(|t| {
            sqlx::query(query)
                .bind(t.amount)
                .bind(t.name)
                .bind(t.merchant_entity_id)
                .bind(t.plaid_transaction_id)
                .bind(t.plaid_account_id())
                .execute(pool)
        });
    join_all(modified_queries).await?;

    Ok(())
}

pub async fn remove_plaid_transactions(pool: &Pool<Sqlite>, removed_transactions: Vec<RemovedTransaction>) -> Result<(), sqlx::Error> {
    let query = r#"
        UPDATE 'transaction'
        SET delete_at=NOW()
        WHERE plaid_transaction_id=$1 AND plaid_accout_id=$2
    "#;

    let removal_queries = removed_transactions
        .into_iter()
        .map(|t| {
            sqlx::query(query)
                .bind(t.transaction_id)
                .bind(t.account_id)
                .execute(pool)
        });
    join_all(removal_queries).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rust_decimal::dec;
    use crate::types::Cents;
    
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
