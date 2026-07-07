use sqlx::{Pool, Sqlite};
use crate::types::PlaidItem;

pub async fn get_plaid_item(pool: &Pool<Sqlite>) -> Result<Vec<PlaidItem>, sqlx::Error> {
    let query = r#"
        SELECT
            pi.item_id,
            pi.access_token
        FROM plaid_item pi
    "#;

    let plaid_items: Vec<PlaidItem> = sqlx::query_as(query)
        .fetch_all(pool)
        .await?;

    Ok(plaid_items)
}

pub async fn insert_plaid_item(
    pool: &Pool<Sqlite>, 
    item_id: String, 
    access_token: &String) -> Result<u64, sqlx::Error> 
{
    let query = r#"
        INSERT INTO plaid_items (item_id, access_token)
        VALUES ($1, $2) 
    "#;

    let res = sqlx::query(query)
        .bind(item_id)
        .bind(access_token)
        .execute(pool)
        .await?;

    Ok(res.rows_affected())
} 
