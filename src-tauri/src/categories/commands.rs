use crate::AppState;
use crate::types::Category;
use std::collections::HashMap;
use crate::categories::queries::get_all_categories;

#[tauri::command]
pub async fn get_category_details(state: tauri::State<'_, AppState>) -> Result<HashMap<String, Category>, String> {
  let mut category_details = state.category_details.lock().await;
  let db = &state.db;

  let details = if let Some(c) = &*category_details {
    c.clone()
  } else {
    let d = get_all_categories(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    *category_details = Some(d.clone());

    d
  };
  
  let category_to_details: HashMap<String, Category> = details
    .iter()
    .map(|d| (d.name.clone(), d.clone()))
    .collect();

  Ok(category_to_details)
}