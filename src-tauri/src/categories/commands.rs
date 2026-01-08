use crate::AppState;
use crate::types::Category;
use crate::categories::queries::{self, get_all_categories};

// TODO: Set up dev db, this will be needed when you have a version of the app locally and you want to develop on top of it

#[tauri::command]
pub async fn get_category_details(state: tauri::State<'_, AppState>) -> Result<Vec<Category>, String> {
  let mut category_details = state.category_details.lock().await;
  let db = &state.db;

  if let Some(c) = &*category_details {
    return Ok(c.clone());
  }

  // TODO: Converting error to string temporarily (since sqlx::Error is not serializable), should create a custom error type
  let details = get_all_categories(&db.0)
    .await
    .map_err(|e| e.to_string())?;

  *category_details = Some(details.clone());

  Ok(details)
}