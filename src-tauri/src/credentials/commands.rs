use keyring::Entry;
use tauri_plugin_store::StoreExt;

const CLIENT_ID_ENTRY: &str = "plaid_client_id";
const SECRET_ENTRY: &str = "plaid_secret";

fn username(app_handle: &tauri::AppHandle) -> Result<String, String> {
    let store = app_handle
        .store("store.json")
        .map_err(|e| format!("Error getting store: {e}"))?;
    let username = store
        .get("username")
        .ok_or("Error fetching username from store")?
        .to_string();
    Ok(username)
}

fn entry(app_handle: &tauri::AppHandle, name: &str) -> Result<Entry, String> {
    let username = username(app_handle)?;
    Entry::new(name, &username).map_err(|e| format!("Error creating {name} entry {e}"))
}

pub fn get_client_id(app_handle: &tauri::AppHandle) -> Result<String, String> {
    entry(app_handle, CLIENT_ID_ENTRY)?
        .get_password()
        .map_err(|e| format!("Error getting client_id password {e}"))
}

pub fn get_secret(app_handle: &tauri::AppHandle) -> Result<String, String> {
    entry(app_handle, SECRET_ENTRY)?
        .get_password()
        .map_err(|e| format!("Error getting secret password {e}"))
}

pub fn set_client_id(app_handle: &tauri::AppHandle, client_id: &str) -> Result<(), String> {
    entry(app_handle, CLIENT_ID_ENTRY)?
        .set_password(client_id)
        .map_err(|e| format!("Error setting client_id password {e}"))
}

pub fn set_secret(app_handle: &tauri::AppHandle, secret: &str) -> Result<(), String> {
    entry(app_handle, SECRET_ENTRY)?
        .set_password(secret)
        .map_err(|e| format!("Error setting secret password {e}"))
}

#[tauri::command]
pub async fn save_plaid_client_id(
    app_handle: tauri::AppHandle,
    client_id: String,
) -> Result<(), String> {
    set_client_id(&app_handle, &client_id)
}

#[tauri::command]
pub async fn save_plaid_secret(
    app_handle: tauri::AppHandle,
    secret: String,
) -> Result<(), String> {
    set_secret(&app_handle, &secret)
}
