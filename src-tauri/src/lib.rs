// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Manager, async_runtime::Mutex};
use tauri_plugin_deep_link::DeepLinkExt;
mod importers {
    pub(crate) mod commands;
    pub(crate) mod bank_of_america;
    pub(crate) mod wells_fargo;
    pub(crate) mod american_express;
    pub(crate) mod types;
}
mod plaid {
    pub(crate) mod commands;
    pub(crate) mod queries;
}
mod transactions {
    pub(crate) mod queries;
}
mod accounts {
    pub(crate) mod commands;
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
    category_details: Mutex<Option<Vec<types::Category>>>,
    link_token: Mutex<Option<String>>
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Add CrabNebula debugger to dev builds
    #[cfg(debug_assertions)]
    let builder = tauri::Builder::default().plugin(tauri_plugin_devtools::init());
    #[cfg(not(debug_assertions))]
    let builder = tauri::Builder::default();

    builder
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Could not find applications data directory");
                
            /* Handle DeepLinks */
            let start_urls = app.deep_link().get_current()?;
            if let Some(urls) = start_urls {
                println!("deep link URLs: {:?}", urls);
            }

            app.deep_link().on_open_url(|event| {
                println!("deep link URLs: {:?}", event.urls());
            });

            /* Initialize DB */
            tauri::async_runtime::block_on(async move {
                let database = db::Database::new(&app_data_dir)
                .await
                .expect("Failed to initialize database");

                app.manage(AppState {
                    db: db::DatabaseState(database.pool),
                    category_details: Mutex::new(None),
                    link_token: Mutex::new(None)
                });
            });
        
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_deep_link::init())
        .invoke_handler(tauri::generate_handler![
            categories::commands::get_category_details,
            accounts::commands::get_all_accounts,
            importers::commands::import_transactions,
            plaid::commands::generate_link_token,
            plaid::commands::generate_access_token_from_hosted_link
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
