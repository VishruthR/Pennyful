use sqlx::{Pool, Sqlite};
use crate::types::Account;

pub async fn get_accounts(pool: &Pool<Sqlite>) -> Result<Vec<Account>, sqlx::Error> {
    let query = "SELECT id, name, bank_id, account_type, initial_balance_cents, current_balance_cents FROM account ORDER BY id";

    let accounts: Vec<Account> = sqlx::query_as(query)
        .fetch_all(pool)
        .await?;

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;
    use crate::types::{AccountType, Cents};

    fn get_expected_accounts() -> Vec<Account> {
        vec![
            Account::new(
                1,
                "Primary Checking".to_owned(),
                1,
                AccountType::Checkings,
                Cents(dec!(1000.00)),
                Cents(dec!(1250.50)),
            ),
            Account::new(
                2,
                "Emergency Fund".to_owned(),
                1,
                AccountType::Savings,
                Cents(dec!(5000.00)),
                Cents(dec!(5200.00)),
            ),
            Account::new(
                3,
                "Joint Checking".to_owned(),
                2,
                AccountType::Checkings,
                Cents(dec!(2500.00)),
                Cents(dec!(2480.75)),
            ),
        ]
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("accounts")))]
    async fn test_get_accounts(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let accounts = get_accounts(&pool).await?;

        assert_eq!(accounts, get_expected_accounts());
        Ok(())
    }
}
