use std::collections::HashMap;
use plaid::model::{AccountBase, AccountSubtype, AccountType};
use sqlx::{Pool, Sqlite};
use crate::types::{Bank, FullAccount};
use crate::types;

pub async fn upsert_new_plaid_accounts(
    pool: &Pool<Sqlite>,
    bank: Bank,
    accounts: Vec<AccountBase>
) -> Result<u32, String> {
    let query = r#"
        INSERT INTO account (
            plaid_account_id,
            name,
            official_name,
            bank_id,
            plaid_item_id,
            account_type,
            initial_balance_cents,
            available_balance_cents,
            current_balance_cents
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT(plaid_account_id) WHERE plaid_account_id IS NOT NULL DO NOTHING
    "#;

    let mut succesfully_inserted: u32 = 0;

    for account in accounts {
        let account_type = match account.type_ {
            AccountType::Depository => {
                match account.subtype.ok_or(format!("Account subtype doesn't exist"))? {
                AccountSubtype::Savings => Ok(types::AccountType::Savings),
                AccountSubtype::Checking => Ok(types::AccountType::Checkings),
                _ => Err("Invalid account subtype")
                }
            },
            AccountType::Credit => Ok(types::AccountType::Credit),
            _ => return Err("Invalid account type".to_string())
        }?;

        

        let current_balance = account.balances.current
            .and_then(types::Cents::from_dollars_f64)
            .unwrap_or_default();
        let available_balance = account.balances.available
            .and_then(types::Cents::from_dollars_f64)
            .unwrap_or_default();

        sqlx::query(query)
            .bind(account.account_id)
            .bind(account.name)
            .bind(account.official_name)
            .bind(bank.id())
            .bind(bank.plaid_item_id())
            .bind(account_type)
            .bind(current_balance)
            .bind(available_balance)
            .bind(current_balance)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to upsert {e}"))?;

        succesfully_inserted += 1;
    }

    Ok(succesfully_inserted)
}

pub async fn get_account_id_by_plaid_id(pool: &Pool<Sqlite>) -> Result<HashMap<String, i64>, sqlx::Error> {
    let query = r#"
        SELECT
            a.plaid_account_id,
            a.id
        FROM account a
        WHERE a.plaid_account_id IS NOT NULL
        ORDER BY a.id
    "#;

    let accounts: Vec<(String, i64)> = sqlx::query_as(query)
        .fetch_all(pool)
        .await?;

    Ok(accounts.into_iter().collect())
}

pub async fn get_full_accounts(pool: &Pool<Sqlite>) -> Result<Vec<FullAccount>, sqlx::Error> {
    let query = r#"
        SELECT
            a.id,
            a.plaid_account_id,
            a.name,
            a.bank_id,
            b.bank_name,
            a.account_type,
            a.initial_balance_cents,
            a.current_balance_cents
        FROM account a
        INNER JOIN bank b ON a.bank_id = b.id
        ORDER BY a.id
    "#;

    let accounts: Vec<FullAccount> = sqlx::query_as(query)
        .fetch_all(pool)
        .await?;

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;
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
    async fn test_get_full_accounts(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let accounts = get_full_accounts(&pool).await?;

        assert_eq!(accounts, get_expected_full_accounts());
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("plaid_sync")))]
    async fn account_map_excludes_manual_accounts(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        // The fixture has a Plaid-linked account (id 1) and a manual one (id 2, no
        // plaid_account_id). Only the linked account should be resolvable.
        let map = get_account_id_by_plaid_id(&pool).await?;

        assert_eq!(map.len(), 1);
        assert_eq!(map.get("plaid-acct-1"), Some(&1));
        Ok(())
    }
}
