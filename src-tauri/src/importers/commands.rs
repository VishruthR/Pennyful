use crate::importers::bank_of_america;
use crate::importers::types::TransactionImport;
use crate::importers::wells_fargo;
use crate::importers::american_express;

/*
Dispatcher for importing transactions from a file

Returns the transactions imported or an error
*/
#[tauri::command]
pub fn import_transactions(file_path: String, bank_name: String) -> Result<Vec<TransactionImport>, String> {
    println!("Importing transactions from file: {}", file_path);
    println!("Bank name: {}", bank_name);
    match bank_name.as_str() {
        "Bank of America" => bank_of_america::parse_csv_statement(file_path).map_err(|e| e.to_string()),
        "Wells Fargo" => wells_fargo::parse_csv_statement(file_path).map_err(|e| e.to_string()),
        "American Express" => american_express::parse_csv_statement(file_path).map_err(|e| e.to_string()),
        _ => Err("Unsupported bank name".to_string()),
    }
}