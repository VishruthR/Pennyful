use chrono::NaiveDate;

use crate::transactions::cents::Cents;
use crate::transactions::transaction_with_account::TransactionWithAccount;
use crate::transactions::queries::{
    get_total_income_by_date_range, get_total_spending_by_date_range,
};
use crate::query::sort_dir::SortDir;
use crate::{categories, transactions, AppState};

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
    sort_dir: Option<SortDir>,
) -> Result<PaginatedSortedTransactionsResponse, String> {
    let db = &state.db;

    let res = transactions::queries::get_paginated_sorted_transactions(
        &db.0, &page, &page_size, &sort_col, &sort_dir,
    )
    .await
    .map_err(|e| format!("Error getting paginated transactions {e}"))?;

    let num_transactions = transactions::queries::get_num_transactions(&db.0)
        .await
        .map_err(|e| format!("Error getting num transactions: {e}"))?;
    // Division with ceiling, doesn't handle negatives properly but we shouldn't see negative
    // numbers
    let num_pages = (num_transactions + page_size - 1) / page_size;
    let prev_page: Option<i64> = if page == 1 { None } else { Some(page - 1) };
    let next_page: Option<i64> = if page >= num_pages {
        None
    } else {
        Some(page + 1)
    };
    let out = PaginatedSortedTransactionsResponse {
        transactions: res,
        curr_page: page,
        next_page,
        prev_page,
        num_pages,
        num_transactions,
    };

    Ok(out)
}

#[tauri::command]
pub async fn get_spending_and_income_by_date_range(
    state: tauri::State<'_, AppState>,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<(Cents, Cents), String> {
    let db = &state.db;

    let income = get_total_income_by_date_range(&db.0, start_date, end_date)
        .await
        .map_err(|e| format!("Error fetching monthly income: {e}"))?;

    let spending = get_total_spending_by_date_range(&db.0, start_date, end_date)
        .await
        .map_err(|e| format!("Error fetching monthly spending: {e}"))?;

    Ok((spending, income))
}

#[tauri::command]
pub async fn update_transaction_category(
    state: tauri::State<'_, AppState>,
    transaction_id: i64,
    category_id: i64,
) -> Result<(), String> {
    let uncategorized = categories::queries::get_uncategorized_category(&state.db.0)
        .await
        .map_err(|e| format!("Error fetching uncategorized category: {e}"))?;

    transactions::queries::update_transaction_category(
        &state.db.0,
        transaction_id,
        category_id,
        *uncategorized.id(),
    )
    .await
    .map_err(|e| format!("Error updating transaction category: {e}"))
}

#[tauri::command]
pub async fn get_transactions_by_category(
    state: tauri::State<'_, AppState>,
    category_name: String,
) -> Result<Vec<TransactionWithAccount>, String> {
    transactions::queries::get_transactions_by_category(&state.db.0, &category_name)
        .await
        .map_err(|e| format!("Error fetching transactions: {e}"))
}

#[tauri::command]
pub async fn get_num_transactions_by_category(
    state: tauri::State<'_, AppState>,
    category_name: String,
) -> Result<i64, String> {
    transactions::queries::get_num_transactions_by_category(&state.db.0, &category_name)
        .await
        .map_err(|e| format!("Error counting transactions: {e}"))
}
