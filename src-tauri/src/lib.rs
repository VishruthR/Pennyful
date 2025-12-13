// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
mod importers {
    pub(crate) mod bank_of_america;
    pub(crate) mod types;
}
mod db;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Could not find applications data directory");

            tauri::async_runtime::block_on(async move {
                let database = db::Database::new(&app_data_dir)
                .await
                .expect("Failed to initialize database");

                app.manage(db::DatabaseState(database.pool));
            });
        
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
