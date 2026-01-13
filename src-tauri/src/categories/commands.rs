use crate::AppState;
use crate::types::Category;
use std::collections::HashMap;
use crate::categories::queries::get_all_categories;

/// Converts a vector of categories to a HashMap keyed by category name
pub fn categories_to_hashmap(categories: Vec<Category>) -> HashMap<String, Category> {
  categories
    .into_iter()
    .map(|c| (c.name.clone(), c))
    .collect()
}

#[tauri::command]
pub async fn get_category_details(state: tauri::State<'_, AppState>) -> Result<HashMap<String, Category>, String> {
  let mut category_details = state.category_details.lock().await;
  let db = &state.db;

  let details = if let Some(c) = &*category_details {
    c.clone()
  } else {
    let d = get_all_categories(&db.0)
        .await
        // TODO: Should map this to custom error that can be serialized and used by FE
        .map_err(|e| e.to_string())?;

    // TODO: Add a method to invalidate this cache when categories are updated
    // Should consider encapsulating category_details modifications in a setter function which automatically handles this
    *category_details = Some(d.clone());

    d
  };
  
  Ok(categories_to_hashmap(details))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Pool, Sqlite};
    use tauri::Manager;
    use tauri::async_runtime::Mutex;
    use crate::db::DatabaseState;

    // Testing a subset of 5 categories to verify the command behavior
    fn get_test_category_names() -> Vec<&'static str> {
        vec!["Uncategorized", "Income", "Groceries", "Entertainment", "Miscellaneous"]
    }

    #[sqlx::test(fixtures(path="../fixtures", scripts("transactions")))]
    async fn test_get_category_details(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let app = tauri::test::mock_app();
        app.manage(AppState {
            db: DatabaseState(pool),
            category_details: Mutex::new(None),
        });

        let result = get_category_details(app.state::<AppState>()).await?;
        
        for name in get_test_category_names() {
            assert!(result.contains_key(name), "Missing category: {}", name);
            let category = result.get(name).unwrap();
            assert_eq!(&category.name, name, "Category name should match key");
        }
        
        Ok(())
    }
}