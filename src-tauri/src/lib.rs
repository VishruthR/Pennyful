// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Manager, async_runtime::Mutex};
mod importers {
    pub(crate) mod bank_of_america;
    pub(crate) mod wells_fargo;
    pub(crate) mod american_express;
    pub(crate) mod types;
}
mod transactions {
    pub(crate) mod queries;
}
mod categories {
    pub(crate) mod commands;
    pub(crate) mod queries;
}
mod db;
mod types;

struct AppState {
    db: db::DatabaseState,
    category_details: Mutex<Option<Vec<types::Category>>>
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

                app.manage(AppState {
                    db: db::DatabaseState(database.pool),
                    category_details: Mutex::new(None)
                });
            });
        
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![categories::commands::get_category_details])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
