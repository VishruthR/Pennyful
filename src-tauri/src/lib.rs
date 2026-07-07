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

// True when one of the delivered deep links is our Plaid completion redirect.
// Matches on the prefix so appended query params (e.g. ?status=success) still count.
fn is_plaid_completion_url(urls: &[tauri::Url]) -> bool {
    urls.iter().any(|u| u.as_str().starts_with("pennyful://plaid-complete"))
}

fn handle_plaid_deep_link(handle: &tauri::AppHandle, urls: Vec<tauri::Url>) {
    if !is_plaid_completion_url(&urls) {
        return;
    }

    let handle = handle.clone();
    tauri::async_runtime::spawn(async move {
        let state = handle.state::<AppState>();
        match plaid::commands::complete_hosted_link(state.inner()).await {
            Ok(access_token) => println!("Plaid access token: {access_token}"),
            Err(e) => eprintln!("Plaid completion failed: {e}"),
        }
    });
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
                
            /* Initialize DB */
            let database = tauri::async_runtime::block_on(db::Database::new(&app_data_dir))
                .expect("Failed to initialize database");

            app.manage(AppState {
                db: db::DatabaseState(database.pool),
                category_details: Mutex::new(None),
                link_token: Mutex::new(None)
            });

            /* Handle DeepLinks */
            let start_urls = app.deep_link().get_current()?;
            if let Some(urls) = start_urls {
                handle_plaid_deep_link(app.handle(), urls);
            }

            let handle = app.handle().clone();
            app.deep_link().on_open_url(move |event| {
                handle_plaid_deep_link(&handle, event.urls());
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

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::Url;

    fn parse(list: &[&str]) -> Vec<Url> {
        list.iter().map(|u| Url::parse(u).expect("valid url")).collect()
    }

    #[test]
    fn matches_completion_url() {
        assert!(is_plaid_completion_url(&parse(&["pennyful://plaid-complete"])));
    }

    #[test]
    fn matches_completion_url_with_query_params() {
        assert!(is_plaid_completion_url(&parse(&[
            "pennyful://plaid-complete?status=success&code=abc"
        ])));
    }

    #[test]
    fn matches_when_completion_url_is_among_several() {
        assert!(is_plaid_completion_url(&parse(&[
            "pennyful://something-else",
            "pennyful://plaid-complete"
        ])));
    }

    #[test]
    fn ignores_other_deep_links() {
        assert!(!is_plaid_completion_url(&parse(&["pennyful://something-else"])));
        assert!(!is_plaid_completion_url(&parse(&["https://plaid.com/plaid-complete"])));
    }

    #[test]
    fn ignores_empty() {
        assert!(!is_plaid_completion_url(&[]));
    }
}
