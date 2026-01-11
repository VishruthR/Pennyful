use crate::AppState;
use crate::types::Category;
use std::collections::HashMap;
use crate::categories::queries::{self, get_all_categories};

// TODO: Set up dev db, this will be needed when you have a version of the app locally and you want to develop on top of it
#[tauri::command]
pub async fn get_category_details(state: tauri::State<'_, AppState>) -> Result<HashMap<String, Category>, String> {
  println!("Called!");
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