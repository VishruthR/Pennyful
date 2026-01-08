use sqlx::{Pool, Sqlite};
use crate::types::Category;

pub async fn get_all_categories(pool: &Pool<Sqlite>) -> Result<Vec<Category>, sqlx::Error> {
    let query = "SELECT id, name, primary_color, secondary_color, icon FROM category ORDER BY id";

    let res: Vec<Category> = sqlx::query_as(query)
        .fetch_all(pool).await?;

    Ok(res)
}