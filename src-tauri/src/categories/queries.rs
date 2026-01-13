use sqlx::{Pool, Sqlite};
use crate::types::Category;

pub async fn get_all_categories(pool: &Pool<Sqlite>) -> Result<Vec<Category>, sqlx::Error> {
    let query = "SELECT id, name, color, icon FROM category ORDER BY id";

    let res: Vec<Category> = sqlx::query_as(query)
        .fetch_all(pool).await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn get_expected_categories() -> Vec<String> {
        // Only test default categories
        return vec![
            "Uncategorized".to_string(),
            "Income".to_string(), 
            "Housing".to_string(),
            "Groceries".to_string(), 
            "Restaurants".to_string(), 
            "Transportation".to_string(), 
            "Healthcare".to_string(), 
            "Savings".to_string(), 
            "Education".to_string(), 
            "Entertainment".to_string(), 
            "Shopping".to_string(), 
            "Hobbies".to_string(),
            "Miscellaneous".to_string()
        ];
    }

    async fn test_get_all_transactions(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let categories = get_all_categories(&pool).await?;

        let category_names: Vec<String> = categories.iter().map(|c| c.name.clone()).collect();
        
        assert_eq!(category_names, get_expected_categories());
        Ok(())
    }
}