use crate::types::{SortDir, TransactionWithAccount};
use crate::{AppState, transactions};

#[tauri::command]
pub async fn get_paginated_sorted_transactions(
    state: tauri::State<'_, AppState>,
    page: i64,
    page_size: i64,
    sort_col: Option<String>,
    sort_dir: Option<SortDir>
) -> Result<Vec<TransactionWithAccount>, String> {
    let db = &state.db;

    let res = transactions::queries::get_paginated_sorted_transactions(&db.0, &page, &page_size, &sort_col, &sort_dir)
         .await
         .map_err(|e| format!("Error getting paginated transactions {e}"))?;

    Ok(res)
}
