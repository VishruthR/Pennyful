use sqlx::{Pool, Sqlite, SqliteConnection};
use crate::types::PlaidItem;

pub async fn get_plaid_item(pool: &Pool<Sqlite>, item_id: &String) -> Result<PlaidItem, sqlx::Error> {
    let query = r#"
        SELECT
            pi.item_id,
            pi.access_token,
            pi.cursor
        FROM plaid_item pi
        WHERE pi.item_id=$1
    "#;

    let plaid_item: PlaidItem = sqlx::query_as(query)
        .bind(item_id)
        .fetch_one(pool)
        .await?;

    Ok(plaid_item)
}

pub async fn get_all_plaid_items(pool: &Pool<Sqlite>) -> Result<Vec<PlaidItem>, sqlx::Error> {
    let query = r#"
        SELECT
            pi.item_id,
            pi.access_token,
            pir.cursor
        FROM plaid_item pi
    "#;

    let plaid_items: Vec<PlaidItem> = sqlx::query_as(query)
        .fetch_all(pool)
        .await?;

    Ok(plaid_items)
}

pub async fn update_plaid_item_cursor(
    conn: &mut SqliteConnection,
    item_id: &String,
    cursor: &Option<String>) -> Result<u64, sqlx::Error>
{
    let query = r#"
        UPDATE plaid_item
        SET cursor=$1
        WHERE item_id=$2
    "#;

    let res = sqlx::query(query)
        .bind(cursor)
        .bind(item_id)
        .execute(&mut *conn)
        .await?;

    Ok(res.rows_affected())
}

pub async fn insert_plaid_item_without_cursor(
    pool: &Pool<Sqlite>, 
    item_id: &String, 
    access_token: &String) -> Result<u64, sqlx::Error> 
{
    let query = r#"
        INSERT INTO plaid_item (item_id, access_token)
        VALUES ($1, $2) 
    "#;

    let res = sqlx::query(query)
        .bind(item_id)
        .bind(access_token)
        .execute(pool)
        .await?;

    Ok(res.rows_affected())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn insert_persists_item_fields(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let affected = insert_plaid_item_without_cursor(&pool, "item-1".to_owned(), &"access-1".to_owned()).await?;
        assert_eq!(affected, 1);

        let rows: Vec<(String, String)> =
            sqlx::query_as("SELECT item_id, access_token FROM plaid_item")
                .fetch_all(&pool)
                .await?;
        assert_eq!(rows, vec![("item-1".to_owned(), "access-1".to_owned())]);
        Ok(())
    }

    #[sqlx::test]
    async fn insert_appends_additional_items(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        insert_plaid_item_without_cursor(&pool, "a".to_owned(), &"t1".to_owned()).await?;
        insert_plaid_item_without_cursor(&pool, "b".to_owned(), &"t2".to_owned()).await?;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM plaid_item")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count.0, 2);
        Ok(())
    }
}

