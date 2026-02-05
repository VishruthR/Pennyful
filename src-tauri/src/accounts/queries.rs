use sqlx::{Pool, Sqlite};
use crate::types::FullAccount;

pub async fn get_full_accounts(pool: &Pool<Sqlite>) -> Result<Vec<FullAccount>, sqlx::Error> {
    let query = r#"
        SELECT 
            a.id,
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
}
