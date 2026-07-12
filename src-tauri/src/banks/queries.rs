use std::collections::HashMap;
use serde_json::{Value};
use sqlx::{Pool, Sqlite};

use crate::types::Bank;

pub async fn upsert_item_from_plaid(
    pool: &Pool<Sqlite>,
    item: &HashMap<String, Value>
) -> Result<u64, sqlx::Error> {
    let query = r#"
        INSERT INTO bank (plaid_item_id, plaid_institution_id, bank_name)
        VALUES (?, ?, ?)
        ON CONFLICT(plaid_item_id) WHERE plaid_item_id IS NOT NULL DO NOTHING
    "#;

    let upsert_res = sqlx::query(query)
        .bind(item.get("item_id").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(item.get("institution_id").and_then(|v| v.as_str()).unwrap_or(""))
        .bind(item.get("institution_name").and_then(|v| v.as_str()).unwrap_or(""))
        .execute(pool)
        .await?;
    
    Ok(upsert_res.rows_affected())
}

pub async fn get_bank_by_item_id(
    pool: &Pool<Sqlite>,
    item_id: &String
) -> Result<Bank, sqlx::Error> {
    let query = r#"
        SELECT
            b.id,
            b.plaid_item_id,
            b.plaid_institution_id,
            b.bank_name
        FROM bank b
        WHERE b.plaid_item_id=$1
    "#;

    let bank: Bank = sqlx::query_as(query)
        .bind(item_id)
        .fetch_one(pool)
        .await?;

    Ok(bank)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn plaid_item(item_id: &str, institution_id: &str, institution_name: &str) -> HashMap<String, Value> {
        HashMap::from([
            ("item_id".to_owned(), json!(item_id)),
            ("institution_id".to_owned(), json!(institution_id)),
            ("institution_name".to_owned(), json!(institution_name)),
        ])
    }

    #[sqlx::test]
    async fn upsert_stores_lookupable_item_id(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        upsert_item_from_plaid(&pool, &plaid_item("item-abc", "ins_1", "Bank of America")).await?;

        // Behavior: the bank is retrievable by the *raw* item_id. This fails if the
        // JSON Value was serialized (stored with surrounding quotes) instead of read
        // as a string.
        let bank = get_bank_by_item_id(&pool, &"item-abc".to_owned()).await?;
        assert_eq!(bank.plaid_item_id(), &Some("item-abc".to_owned()));

        let bank_name: String = sqlx::query_scalar("SELECT bank_name FROM bank WHERE plaid_item_id = ?")
            .bind("item-abc")
            .fetch_one(&pool)
            .await?;
        assert_eq!(bank_name, "Bank of America");
        Ok(())
    }

    #[sqlx::test]
    async fn upsert_is_idempotent_on_item_id(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let item = plaid_item("item-abc", "ins_1", "Bank of America");
        assert_eq!(upsert_item_from_plaid(&pool, &item).await?, 1);
        // Re-linking the same item is a no-op rather than a duplicate bank.
        assert_eq!(upsert_item_from_plaid(&pool, &item).await?, 0);

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM bank")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count, 1);
        Ok(())
    }
}
