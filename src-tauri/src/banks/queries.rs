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
