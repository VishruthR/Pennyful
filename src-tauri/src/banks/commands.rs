use crate::accounts::queries as account_queries;
use crate::banks::queries;
use crate::types::LinkedInstitution;
use crate::AppState;

#[tauri::command]
pub async fn get_linked_institutions(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<LinkedInstitution>, String> {
    let db = &state.db;

    let banks = queries::get_all_plaid_banks(&db.0)
        .await
        .map_err(|e| e.to_string())?;
    let counts = account_queries::get_account_counts_by_item(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    let institutions = banks
        .into_iter()
        .filter_map(|bank| {
            let item_id = bank.plaid_item_id().clone()?;
            let account_count = counts.get(&item_id).copied().unwrap_or(0);
            Some(LinkedInstitution {
                item_id,
                institution_name: bank.bank_name().clone(),
                account_count,
            })
        })
        .collect();

    Ok(institutions)
}
