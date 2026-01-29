use crate::AppState;
use crate::types::FullAccount;
use crate::accounts::queries::get_full_accounts;

#[tauri::command]
pub async fn get_all_accounts(state: tauri::State<'_, AppState>) -> Result<Vec<FullAccount>, String> {
    let db = &state.db;

    let accounts = get_full_accounts(&db.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Pool, Sqlite};
    use tauri::Manager;
    use tauri::async_runtime::Mutex;
    use rust_decimal::dec;
    use crate::db::DatabaseState;
    use crate::types::{AccountType, Cents};

    fn get_expected_full_accounts() -> Vec<FullAccount> {
        vec![
            FullAccount::new(
                1,
                "Primary Checking".to_owned(),
                1,
                "Bank of America".to_owned(),
                AccountType::Checkings,
                Cents(dec!(1000.00)),
                Cents(dec!(1250.50)),
            ),
            FullAccount::new(
                2,
                "Emergency Fund".to_owned(),
                1,
                "Bank of America".to_owned(),
                AccountType::Savings,
                Cents(dec!(5000.00)),
                Cents(dec!(5200.00)),
            ),
            FullAccount::new(
                3,
                "Joint Checking".to_owned(),
                2,
                "Wells Fargo".to_owned(),
                AccountType::Checkings,
                Cents(dec!(2500.00)),
                Cents(dec!(2480.75)),
            ),
        ]
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("accounts")))]
    async fn test_get_all_accounts(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let app = tauri::test::mock_app();
        app.manage(AppState {
            db: DatabaseState(pool),
            category_details: Mutex::new(None),
        });

        let result = get_all_accounts(app.state::<AppState>()).await?;

        assert_eq!(result, get_expected_full_accounts());
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("accounts")))]
    async fn test_get_all_accounts_has_bank_name(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let app = tauri::test::mock_app();
        app.manage(AppState {
            db: DatabaseState(pool),
            category_details: Mutex::new(None),
        });

        let result = get_all_accounts(app.state::<AppState>()).await?;
        
        assert_eq!(result[0].bank_name, "Bank of America");
        assert_eq!(result[2].bank_name, "Wells Fargo");
        Ok(())
    }
}
