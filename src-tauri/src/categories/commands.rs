use crate::categories::queries;
use crate::categories::queries::get_all_categories;
use crate::types::{Category, CategoryOverview};
use crate::AppState;
use std::collections::HashMap;

/// Converts a vector of categories to a HashMap keyed by category name
pub fn categories_to_hashmap(categories: Vec<Category>) -> HashMap<String, Category> {
    categories
        .into_iter()
        .map(|c| (c.name.clone(), c))
        .collect()
}

#[tauri::command]
pub async fn get_category_details(
    state: tauri::State<'_, AppState>,
) -> Result<HashMap<String, Category>, String> {
    let details = get_all_categories(&state.db.0)
        .await
        // TODO: Should map this to custom error that can be serialized and used by FE
        .map_err(|e| e.to_string())?;

    Ok(categories_to_hashmap(details))
}

#[tauri::command]
pub async fn get_category_overviews(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<CategoryOverview>, String> {
    queries::get_category_overviews(&state.db.0)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_category_budget(
    state: tauri::State<'_, AppState>,
    category_id: i64,
    amount_cents: i64,
) -> Result<(), String> {
    queries::upsert_budget(&state.db.0, category_id, amount_cents)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_category(
    state: tauri::State<'_, AppState>,
    name: String,
    color: String,
    icon: Option<String>,
    budget_cents: Option<i64>,
) -> Result<i64, String> {
    queries::create_category(&state.db.0, &name, &color, icon.as_deref(), budget_cents)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_category(
    state: tauri::State<'_, AppState>,
    id: i64,
    name: String,
    color: String,
    icon: Option<String>,
    budget_cents: Option<i64>,
) -> Result<(), String> {
    queries::update_category(&state.db.0, id, &name, &color, icon.as_deref(), budget_cents)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_category(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<(), String> {
    queries::delete_category(&state.db.0, id)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DatabaseState;
    use sqlx::{Pool, Sqlite};
    use tauri::async_runtime::Mutex;
    use tauri::Manager;

    // Testing a subset of 5 categories to verify the command behavior
    fn get_test_category_names() -> Vec<&'static str> {
        vec![
            "Uncategorized",
            "Income",
            "Groceries",
            "Entertainment",
            "Miscellaneous",
        ]
    }

    async fn test_get_category_details(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let app = tauri::test::mock_app();
        app.manage(AppState {
            db: DatabaseState(pool),
            link_token: Mutex::new(None),
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
