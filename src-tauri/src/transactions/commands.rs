use crate::types::{SortDir, TransactionWithAccount};
use crate::{AppState, transactions};

#[derive(serde::Serialize)]
pub struct PaginatedSortedTransactionsResponse {
    pub transactions: Vec<TransactionWithAccount>,
    pub curr_page: i64,
    pub next_page: Option<i64>,
    pub prev_page: Option<i64>,
    pub num_pages: i64,
    pub num_transactions: i64,
}

#[tauri::command]
pub async fn get_paginated_sorted_transactions(
    state: tauri::State<'_, AppState>,
    page: i64,
    page_size: i64,
    sort_col: Option<String>,
    sort_dir: Option<SortDir>
) -> Result<PaginatedSortedTransactionsResponse, String> {
    let db = &state.db;

    let res = transactions::queries::get_paginated_sorted_transactions(&db.0, &page, &page_size, &sort_col, &sort_dir)
         .await
         .map_err(|e| format!("Error getting paginated transactions {e}"))?;

    let num_transactions = transactions::queries::get_num_transactions(&db.0)
        .await
        .map_err(|e| format!("Error getting num transactions: {e}"))?;
    // Division with ceiling, doesn't handle negatives properly but we shouldn't see negative
    // numbers
    let num_pages = (num_transactions + page_size - 1) / page_size;
    let prev_page: Option<i64> = if page == 1 { None } else { Some(page - 1) };
    let next_page: Option<i64> = if page >= num_pages { None } else { Some(page + 1) };
    let out = PaginatedSortedTransactionsResponse {
        transactions: res,
        curr_page: page,
        next_page: next_page,
        prev_page: prev_page,
        num_pages: num_pages,
        num_transactions: num_transactions
    };

    Ok(out)
}
