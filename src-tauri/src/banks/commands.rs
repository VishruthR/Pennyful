use crate::accounts::queries as account_queries;
use crate::types::LinkedInstitution;
use crate::AppState;

#[tauri::command]
pub async fn get_linked_institutions(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<LinkedInstitution>, String> {
    let db = &state.db;

    let account_counts = account_queries::get_account_counts_by_item(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(account_counts)
}
