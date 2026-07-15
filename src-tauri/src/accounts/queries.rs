use crate::types::{self, LinkedInstitution};
use crate::types::{Account, Bank, FullAccount};
use plaid::model::{AccountBase, AccountSubtype, AccountType};
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;

pub async fn insert_new_plaid_accounts(
    pool: &Pool<Sqlite>,
    bank: Bank,
    accounts: Vec<AccountBase>,
) -> Result<u32, String> {
    let query = r#"
        INSERT INTO account (
            plaid_account_id,
            name,
            official_name,
            bank_id,
            account_type,
            initial_balance_cents,
            available_balance_cents,
            current_balance_cents
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT(plaid_account_id) WHERE plaid_account_id IS NOT NULL DO NOTHING
    "#;

    let mut successfully_inserted: u32 = 0;

    for account in accounts {
        let account_type = match account.type_ {
            AccountType::Depository => {
                match account
                    .subtype
                    .ok_or(format!("Account subtype doesn't exist"))?
                {
                    AccountSubtype::Savings => Ok(types::AccountType::Savings),
                    AccountSubtype::Checking => Ok(types::AccountType::Checkings),
                    _ => Err("Invalid account subtype"),
                }
            }
            AccountType::Credit => Ok(types::AccountType::Credit),
            _ => return Err("Invalid account type".to_string()),
        }?;

        let current_balance = account
            .balances
            .current
            .and_then(types::Cents::from_dollars_f64)
            .unwrap_or_default();
        let available_balance = account
            .balances
            .available
            .and_then(types::Cents::from_dollars_f64)
            .unwrap_or_default();

        let res = sqlx::query(query)
            .bind(account.account_id)
            .bind(account.name)
            .bind(account.official_name)
            .bind(bank.id())
            .bind(account_type)
            .bind(current_balance)
            .bind(available_balance)
            .bind(current_balance)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to insert {e}"))?;

        successfully_inserted += res.rows_affected() as u32;
    }

    Ok(successfully_inserted)
}

pub async fn get_account_id_by_plaid_id(
    pool: &Pool<Sqlite>,
) -> Result<HashMap<String, i64>, sqlx::Error> {
    let query = r#"
        SELECT
            a.plaid_account_id,
            a.id
        FROM account a
        WHERE a.plaid_account_id IS NOT NULL
        ORDER BY a.id
    "#;

    let accounts: Vec<(String, i64)> = sqlx::query_as(query).fetch_all(pool).await?;

    Ok(accounts.into_iter().collect())
}

pub async fn get_account_counts_by_item(
    pool: &Pool<Sqlite>,
) -> Result<Vec<LinkedInstitution>, sqlx::Error> {
    let query = r#"
        SELECT
            b.bank_name AS institution_name
            b.plaid_item_id AS item_id,
            COUNT(*) AS account_count
        FROM account a
        INNER JOIN bank b ON a.bank_id = b.id
        WHERE b.plaid_item_id IS NOT NULL
        GROUP BY b.plaid_item_id
    "#;

    let account_counts: Vec<LinkedInstitution> = sqlx::query_as(query).fetch_all(pool).await?;

    Ok(account_counts)
}

pub async fn get_accounts_of_item(
    pool: &Pool<Sqlite>,
    item_id: &String,
) -> Result<Vec<Account>, sqlx::Error> {
    let query = r#"
        SELECT
            a.id,
            a.plaid_account_id,
            a.name,
            a.official_name,
            a.bank_id,
            a.account_type,
            a.initial_balance_cents,
            a.available_balance_cents,
            a.current_balance_cents
        FROM account a
        INNER JOIN bank b ON a.bank_id = b.id
        WHERE b.plaid_item_id=?
        ORDER BY a.id
    "#;

    let accounts: Vec<Account> = sqlx::query_as(query).bind(item_id).fetch_all(pool).await?;

    Ok(accounts)
}

pub async fn get_full_accounts(pool: &Pool<Sqlite>) -> Result<Vec<FullAccount>, sqlx::Error> {
    let query = r#"
        SELECT
            a.id,
            a.plaid_account_id,
            a.name,
            a.official_name,
            a.bank_id,
            b.bank_name,
            a.account_type,
            a.initial_balance_cents,
            a.available_balance_cents,
            a.current_balance_cents
        FROM account a
        INNER JOIN bank b ON a.bank_id = b.id
        ORDER BY a.id
    "#;

    let accounts: Vec<FullAccount> = sqlx::query_as(query).fetch_all(pool).await?;

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AccountType, Cents};
    use rust_decimal::dec;
    use serde_json::json;

    fn plaid_account(account_id: &str, type_: &str, subtype: Option<&str>) -> AccountBase {
        serde_json::from_value(json!({
            "account_id": account_id,
            "balances": { "available": 100.0, "current": 150.0 },
            "name": account_id,
            "type": type_,
            "subtype": subtype,
        }))
        .expect("valid AccountBase")
    }

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
    async fn test_get_full_accounts(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let accounts = get_full_accounts(&pool).await?;

        assert_eq!(accounts, get_expected_full_accounts());
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("plaid_sync")))]
    async fn account_map_excludes_manual_accounts(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // The fixture has a Plaid-linked account (id 1) and a manual one (id 2, no
        // plaid_account_id). Only the linked account should be resolvable.
        let map = get_account_id_by_plaid_id(&pool).await?;

        assert_eq!(map.len(), 1);
        assert_eq!(map.get("plaid-acct-1"), Some(&1));
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("accounts_of_item")))]
    async fn get_accounts_of_item_returns_only_that_items_accounts(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let accounts = get_accounts_of_item(&pool, &"item-1".to_owned()).await?;

        let ids: Vec<i64> = accounts.iter().map(|a| a.id).collect();
        assert_eq!(ids, vec![1, 2]);
        assert!(accounts
            .iter()
            .all(|a| a.plaid_item_id == Some("item-1".to_owned())));
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("accounts_of_item")))]
    async fn get_accounts_of_item_is_empty_for_unknown_item(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let accounts = get_accounts_of_item(&pool, &"does-not-exist".to_owned()).await?;

        assert!(accounts.is_empty());
        Ok(())
    }

    async fn seed_bank(pool: &Pool<Sqlite>) -> Bank {
        sqlx::query("INSERT INTO bank (id, plaid_item_id, bank_name) VALUES (1, 'item-1', 'Test Bank')")
            .execute(pool)
            .await
            .expect("seed bank");
        Bank::new(1, Some("item-1".to_owned()), None, "Test Bank".to_owned())
    }

    #[sqlx::test]
    async fn insert_maps_plaid_types_to_account_types(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bank = seed_bank(&pool).await;

        let inserted = insert_new_plaid_accounts(
            &pool,
            bank,
            vec![
                plaid_account("acct-check", "depository", Some("checking")),
                plaid_account("acct-save", "depository", Some("savings")),
                plaid_account("acct-credit", "credit", None),
            ],
        )
        .await?;
        assert_eq!(inserted, 3);

        let rows: Vec<(String, String)> = sqlx::query_as(
            "SELECT plaid_account_id, account_type FROM account ORDER BY plaid_account_id",
        )
        .fetch_all(&pool)
        .await?;
        assert_eq!(
            rows,
            vec![
                ("acct-check".to_owned(), "CHECKINGS".to_owned()),
                ("acct-credit".to_owned(), "CREDIT".to_owned()),
                ("acct-save".to_owned(), "SAVINGS".to_owned()),
            ]
        );
        Ok(())
    }

    #[sqlx::test]
    async fn insert_skips_already_linked_accounts(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bank = seed_bank(&pool).await;

        let first = insert_new_plaid_accounts(
            &pool,
            Bank::new(1, Some("item-1".to_owned()), None, "Test Bank".to_owned()),
            vec![plaid_account("acct-1", "depository", Some("checking"))],
        )
        .await?;
        assert_eq!(first, 1);

        // Re-linking the same account must not duplicate it or count as inserted.
        let second = insert_new_plaid_accounts(
            &pool,
            bank,
            vec![plaid_account("acct-1", "depository", Some("checking"))],
        )
        .await?;
        assert_eq!(second, 0);

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM account")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count, 1);
        Ok(())
    }

    #[sqlx::test]
    async fn insert_rejects_unsupported_account_shapes(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bank = || Bank::new(1, Some("item-1".to_owned()), None, "Test Bank".to_owned());
        seed_bank(&pool).await;

        // Depository account with no subtype cannot be classified.
        assert!(insert_new_plaid_accounts(
            &pool,
            bank(),
            vec![plaid_account("acct-1", "depository", None)],
        )
        .await
        .is_err());

        // Depository subtype that is neither checking nor savings is unsupported.
        assert!(insert_new_plaid_accounts(
            &pool,
            bank(),
            vec![plaid_account("acct-2", "depository", Some("cd"))],
        )
        .await
        .is_err());

        // Account types other than depository/credit are unsupported.
        assert!(insert_new_plaid_accounts(
            &pool,
            bank(),
            vec![plaid_account("acct-3", "loan", None)],
        )
        .await
        .is_err());
        Ok(())
    }
}
