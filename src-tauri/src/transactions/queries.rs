use crate::plaid::plaid_transaction::PlaidTransaction;
use crate::query::sort_dir::SortDir;
use crate::transactions::{dollars::Dollars, transaction_with_account::TransactionWithAccount};
use ::plaid::model::RemovedTransaction;
use chrono::NaiveDate;
use sqlx::{Pool, QueryBuilder, Sqlite, SqliteConnection};
use std::collections::HashMap;

pub async fn get_total_income_by_date_range(
    pool: &Pool<Sqlite>,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Dollars, sqlx::Error> {
    let query = r#"
        SELECT
            SUM(t.amount_cents) AS amount
        FROM 'transaction' t
        JOIN category c on t.category_id=c.id
        WHERE c.name = 'Income' 
            AND date(t.date) >= date($1) 
            AND date(t.date) <= date($2)
    "#;

    let res: Dollars = sqlx::query_scalar(query)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(pool)
        .await?;

    Ok(res)
}

pub async fn get_total_spending_by_date_range(
    pool: &Pool<Sqlite>,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Dollars, sqlx::Error> {
    let query = r#"
        SELECT
            SUM(t.amount_cents) AS amount
        FROM 'transaction' t
        JOIN category c on t.category_id=c.id
        WHERE c.name != 'Income' 
            AND date(t.date) >= date($1) 
            AND date(t.date) <= date($2)
    "#;

    let res: Dollars = sqlx::query_scalar(query)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(pool)
        .await?;

    Ok(res)
}

pub async fn get_transactions_by_category(
    pool: &Pool<Sqlite>,
    category_name: &String,
) -> Result<Vec<TransactionWithAccount>, sqlx::Error> {
    let query = r#"
        SELECT
            t.id,
            t.plaid_transaction_id,
            t.name,
            t.merchant_entity_id,
            t.amount_cents,
            t.date,
            t.pending,
            t.deleted_at,
            t.account_id,
            t.category_id,
            a.name AS account_name,
            b.plaid_institution_id AS bank_institution_id,
            c.name AS category_name,
            c.color AS category_color,
            c.icon AS category_icon
        FROM 'transaction' t
        JOIN account a ON t.account_id=a.id
        JOIN bank b ON a.bank_id=b.id
        JOIN category c ON t.category_id=c.id
        WHERE t.deleted_at IS NULL AND c.name = ?
    "#;

    let res: Vec<TransactionWithAccount> = sqlx::query_as(query)
        .bind(category_name)
        .fetch_all(pool)
        .await?;

    Ok(res)
}

pub async fn get_num_transactions(pool: &Pool<Sqlite>) -> Result<i64, sqlx::Error> {
    let res: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM 'transaction' WHERE deleted_at IS NULL")
            .fetch_one(pool)
            .await?;

    Ok(res)
}

pub async fn get_num_transactions_by_category(
    pool: &Pool<Sqlite>,
    category_name: &String,
) -> Result<i64, sqlx::Error> {
    let res: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM 'transaction' t
        JOIN category c ON t.category_id=c.id
        WHERE t.deleted_at IS NULL AND c.name = ?
        "#,
    )
    .bind(category_name)
    .fetch_one(pool)
    .await?;

    Ok(res)
}

pub async fn get_paginated_sorted_transactions(
    pool: &Pool<Sqlite>,
    page: &i64,
    page_size: &i64,
    sort_col: &Option<String>,
    sort_dir: &Option<SortDir>,
) -> Result<Vec<TransactionWithAccount>, sqlx::Error> {
    let offset = std::cmp::max(page - 1, 0) * page_size;

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
        SELECT
            t.id,
            t.plaid_transaction_id,
            t.name,
            t.merchant_entity_id,
            t.amount_cents,
            t.date,
            t.pending,
            t.deleted_at,
            t.account_id,
            t.category_id,
            a.name AS account_name,
            b.plaid_institution_id AS bank_institution_id,
            c.name AS category_name,
            c.color AS category_color,
            c.icon AS category_icon
        FROM 'transaction' t
        JOIN account a ON t.account_id=a.id
        JOIN bank b ON a.bank_id=b.id
        JOIN category c ON t.category_id=c.id
        WHERE t.deleted_at IS NULL
    "#,
    );

    query_builder.push(" ORDER BY ");
    if let Some(col) = sort_col.as_deref() {
        let sort_col_final = match col {
            "date" => "t.date",
            "account" => "a.name",
            "name" => "t.name",
            "amount" => "t.amount_cents",
            _ => return Err(sqlx::Error::Protocol("Invalid sort column".into())),
        };

        let sort_dir_final = match sort_dir.unwrap_or(SortDir::Asc) {
            SortDir::Asc => "ASC",
            SortDir::Desc => "DESC",
        };

        query_builder.push(sort_col_final);
        query_builder.push(" ");
        query_builder.push(sort_dir_final);
        query_builder.push(", ");
    }
    // Stable tiebreaker so LIMIT/OFFSET paging never overlaps or drops rows.
    query_builder.push("t.id");

    query_builder.push(" LIMIT ");
    query_builder.push_bind(page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let transactions: Vec<TransactionWithAccount> = query_builder
        .build_query_as::<TransactionWithAccount>()
        .fetch_all(pool)
        .await?;

    Ok(transactions)
}

pub async fn add_plaid_transactions(
    conn: &mut SqliteConnection,
    new_transactions: Vec<PlaidTransaction>,
    default_category: i64,
    rules: &HashMap<String, i64>,
) -> Result<u64, sqlx::Error> {
    if new_transactions.is_empty() {
        return Ok(0);
    }
    let num_transactions = new_transactions.len() as u64;

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "INSERT INTO 'transaction' (plaid_transaction_id, name, merchant_entity_id, amount_cents, date, pending, account_id, category_id) "
    );

    query_builder.push_values(new_transactions, |mut b, t| {
        let account_id = *t.account_id();

        let category_id = t
            .name
            .as_deref()
            .and_then(|name| rules.get(name))
            .copied()
            .unwrap_or(default_category);

        b.push_bind(t.plaid_transaction_id)
            .push_bind(t.name.unwrap_or("".to_string()))
            .push_bind(t.merchant_entity_id)
            .push_bind(t.amount)
            .push_bind(t.date)
            .push_bind(t.pending)
            .push_bind(account_id)
            .push_bind(category_id);
    });
    query_builder.push(
        " ON CONFLICT(plaid_transaction_id) WHERE plaid_transaction_id IS NOT NULL DO NOTHING",
    );

    let query = query_builder.build();
    let res = query.execute(&mut *conn).await?;

    Ok(num_transactions - res.rows_affected())
}

pub async fn update_transaction_category(
    pool: &Pool<Sqlite>,
    transaction_id: i64,
    category_id: i64,
    uncategorized_id: i64,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let _res = sqlx::query(r#"UPDATE 'transaction' SET category_id=? WHERE id=?"#)
        .bind(category_id)
        .bind(transaction_id)
        .execute(&mut *tx)
        .await?;

    // Remember this decision so future transactions from the same merchant inherit
    let name: Option<String> = sqlx::query_scalar(r#"SELECT name FROM 'transaction' WHERE id=?"#)
        .bind(transaction_id)
        .fetch_optional(&mut *tx)
        .await?;

    if let Some(name) = name {
        if !name.is_empty() {
            if category_id == uncategorized_id {
                delete_category_rule(&mut tx, &name).await?;
            } else {
                upsert_category_rule(&mut tx, &name, category_id).await?;
            }
        }
    }

    tx.commit().await?;

    Ok(())
}

pub async fn get_category_rules(pool: &Pool<Sqlite>) -> Result<HashMap<String, i64>, sqlx::Error> {
    let rules: Vec<(String, i64)> =
        sqlx::query_as(r#"SELECT name, category_id FROM transaction_category_rule"#)
            .fetch_all(pool)
            .await?;

    Ok(rules.into_iter().collect())
}

async fn upsert_category_rule(
    conn: &mut SqliteConnection,
    name: &str,
    category_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO transaction_category_rule (name, category_id)
        VALUES (?, ?)
        ON CONFLICT(name) DO UPDATE SET
            category_id=excluded.category_id,
            updated_at=datetime('now')
        "#,
    )
    .bind(name)
    .bind(category_id)
    .execute(&mut *conn)
    .await?;

    Ok(())
}

async fn delete_category_rule(conn: &mut SqliteConnection, name: &str) -> Result<(), sqlx::Error> {
    sqlx::query(r#"DELETE FROM transaction_category_rule WHERE name=?"#)
        .bind(name)
        .execute(&mut *conn)
        .await?;

    Ok(())
}

pub async fn modify_plaid_transactions(
    conn: &mut SqliteConnection,
    modified_transactions: Vec<PlaidTransaction>,
) -> Result<(), sqlx::Error> {
    let query = r#"
        UPDATE 'transaction'
        SET amount_cents=?,
            date=?,
            name=?,
            merchant_entity_id=?,
            pending=?
        WHERE plaid_transaction_id=?
    "#;

    for t in modified_transactions {
        sqlx::query(query)
            .bind(t.amount)
            .bind(t.date)
            .bind(t.name.unwrap_or_default())
            .bind(t.merchant_entity_id)
            .bind(t.pending)
            .bind(t.plaid_transaction_id)
            .execute(&mut *conn)
            .await?;
    }

    Ok(())
}

pub async fn remove_plaid_transactions(
    conn: &mut SqliteConnection,
    removed_transactions: Vec<RemovedTransaction>,
) -> Result<(), sqlx::Error> {
    let query = r#"
        UPDATE 'transaction'
        SET deleted_at=date('now')
        WHERE plaid_transaction_id=?
    "#;

    for t in removed_transactions {
        sqlx::query(query)
            .bind(t.transaction_id)
            .execute(&mut *conn)
            .await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn all_transactions(
        pool: &Pool<Sqlite>,
    ) -> Result<Vec<TransactionWithAccount>, sqlx::Error> {
        get_paginated_sorted_transactions(
            pool,
            &1,
            &10,
            &Some("name".to_owned()),
            &Some(SortDir::Asc),
        )
        .await
    }

    fn plaid_txn(
        plaid_id: &str,
        name: &str,
        amount_dollars: f64,
        pending: bool,
    ) -> PlaidTransaction {
        PlaidTransaction::new(
            Some(plaid_id.to_owned()),
            Some(name.to_owned()),
            None,
            Dollars::from_dollars_f64(amount_dollars).unwrap(),
            NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
            pending,
            "plaid-acct-1".to_owned(),
            Some(1),
            None,
        )
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn get_income_by_date_range_calculates_correctly(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // With no `Income` transactions, we should see 0
        let start_date = NaiveDate::parse_from_str("2025-12-01", "%Y-%m-%d")?;
        let end_date = NaiveDate::parse_from_str("2025-12-31", "%Y-%m-%d")?;
        let income = get_total_income_by_date_range(&pool, start_date, end_date).await?;
        assert_eq!(income, Dollars::from_dollars_f64(0_f64).unwrap());

        update_transaction_category(&pool, 1, 2, 1).await?;
        let income = get_total_income_by_date_range(&pool, start_date, end_date).await?;
        assert_eq!(income, Dollars::from_dollars_f64(-5.77_f64).unwrap());

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn get_spending_by_date_range_calculates_correctly(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // No transactions in date range should return 0
        let start_date = NaiveDate::parse_from_str("2026-12-01", "%Y-%m-%d")?;
        let end_date = NaiveDate::parse_from_str("2026-12-31", "%Y-%m-%d")?;
        let income = get_total_spending_by_date_range(&pool, start_date, end_date).await?;
        assert_eq!(income, Dollars::from_dollars_f64(0_f64).unwrap());

        let start_date = NaiveDate::parse_from_str("2025-12-01", "%Y-%m-%d")?;
        let end_date = NaiveDate::parse_from_str("2025-12-31", "%Y-%m-%d")?;
        let income = get_total_spending_by_date_range(&pool, start_date, end_date).await?;
        assert_eq!(income, Dollars::from_dollars_f64(-19.27_f64).unwrap());

        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("plaid_sync")))]
    async fn add_persists_new_transactions(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = pool.acquire().await?;
        let skipped = add_plaid_transactions(
            &mut conn,
            vec![
                plaid_txn("txn-1", "Coffee", -4.50, false),
                plaid_txn("txn-2", "Books", -20.00, false),
            ],
            1,
            &HashMap::new(),
        )
        .await?;
        assert_eq!(skipped, 0);

        let transactions = all_transactions(&pool).await?;
        assert_eq!(
            transactions
                .iter()
                .filter(|t| t.plaid_transaction_id().is_some())
                .count(),
            2
        );

        let coffee = transactions
            .iter()
            .find(|t| t.plaid_transaction_id().as_deref() == Some("txn-1"))
            .expect("added transaction should be queryable");
        assert_eq!(coffee.name, "Coffee");
        assert_eq!(coffee.amount, Dollars::from_dollars_f64(-4.50).unwrap());
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("plaid_sync")))]
    async fn add_skips_duplicate_plaid_transaction_ids(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = pool.acquire().await?;
        let first = add_plaid_transactions(
            &mut conn,
            vec![plaid_txn("txn-1", "Coffee", -4.50, false)],
            1,
            &HashMap::new(),
        )
        .await?;
        assert_eq!(first, 0);

        // Re-syncing the same transaction (Plaid resends) must not duplicate it, and
        // the whole batch is reported as skipped.
        let second = add_plaid_transactions(
            &mut conn,
            vec![plaid_txn("txn-1", "Coffee CHANGED", -9.99, false)],
            1,
            &HashMap::new(),
        )
        .await?;
        assert_eq!(second, 1);

        let transactions = all_transactions(&pool).await?;
        let matching: Vec<_> = transactions
            .iter()
            .filter(|t| t.plaid_transaction_id().as_deref() == Some("txn-1"))
            .collect();
        assert_eq!(
            matching.len(),
            1,
            "duplicate should not create a second row"
        );
        assert_eq!(
            matching[0].name, "Coffee",
            "existing row should be left untouched"
        );
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("plaid_sync")))]
    async fn modify_updates_matching_transaction(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = pool.acquire().await?;
        add_plaid_transactions(
            &mut conn,
            vec![plaid_txn("txn-1", "Pending Coffee", -4.50, true)],
            1,
            &HashMap::new(),
        )
        .await?;

        let mut posted = plaid_txn("txn-1", "Posted Coffee", -5.25, false);
        posted.date = NaiveDate::from_ymd_opt(2026, 1, 20).unwrap();
        modify_plaid_transactions(&mut conn, vec![posted]).await?;

        let transactions = all_transactions(&pool).await?;
        let txn = transactions
            .iter()
            .find(|t| t.plaid_transaction_id().as_deref() == Some("txn-1"))
            .expect("transaction should exist");
        assert_eq!(txn.name, "Posted Coffee");
        assert_eq!(txn.amount, Dollars::from_dollars_f64(-5.25).unwrap());
        assert_eq!(txn.date, NaiveDate::from_ymd_opt(2026, 1, 20).unwrap());
        assert!(!txn.pending, "pending should flip to posted");
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("plaid_sync")))]
    async fn remove_soft_deletes_and_hides_transaction(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = pool.acquire().await?;
        add_plaid_transactions(
            &mut conn,
            vec![plaid_txn("txn-1", "Coffee", -4.50, false)],
            1,
            &HashMap::new(),
        )
        .await?;

        remove_plaid_transactions(
            &mut conn,
            vec![RemovedTransaction {
                transaction_id: "txn-1".to_owned(),
                ..Default::default()
            }],
        )
        .await?;

        let transactions = all_transactions(&pool).await?;
        assert!(
            transactions
                .iter()
                .all(|t| t.plaid_transaction_id().as_deref() != Some("txn-1")),
            "removed transaction should not appear in get_transactions"
        );

        let deleted_at: Option<String> = sqlx::query_scalar(
            "SELECT deleted_at FROM 'transaction' WHERE plaid_transaction_id = ?",
        )
        .bind("txn-1")
        .fetch_one(&pool)
        .await?;
        assert!(
            deleted_at.is_some(),
            "row should still exist with deleted_at set"
        );
        Ok(())
    }

    fn ids(transactions: &[TransactionWithAccount]) -> Vec<i64> {
        transactions.iter().map(|t| *t.id()).collect()
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn num_by_category_counts_only_matching_category(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // All 4 fixture transactions are category_id = 1 ("Uncategorized").
        assert_eq!(
            get_num_transactions_by_category(&pool, &"Uncategorized".to_owned()).await?,
            4
        );

        // Reassigning one to another category drops the uncategorized count.
        update_transaction_category(&pool, 1, 4, 1).await?;
        assert_eq!(
            get_num_transactions_by_category(&pool, &"Uncategorized".to_owned()).await?,
            3
        );
        assert_eq!(
            get_num_transactions_by_category(&pool, &"Groceries".to_owned()).await?,
            1
        );
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn paginated_walks_pages_without_overlap(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(get_num_transactions(&pool).await?, 4);

        // Sort by amount (unique values) so paging is deterministic.
        // Ascending amounts: id2=-10.90, id1=-5.77, id3=-1.90, id4=-0.70
        let sort_col = Some("amount".to_owned());
        let page1 =
            get_paginated_sorted_transactions(&pool, &1, &2, &sort_col, &Some(SortDir::Asc))
                .await?;
        let page2 =
            get_paginated_sorted_transactions(&pool, &2, &2, &sort_col, &Some(SortDir::Asc))
                .await?;
        let page3 =
            get_paginated_sorted_transactions(&pool, &3, &2, &sort_col, &Some(SortDir::Asc))
                .await?;

        assert_eq!(ids(&page1), vec![2, 1]);
        assert_eq!(ids(&page2), vec![3, 4]);
        assert!(page3.is_empty(), "page past the end should be empty");
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn paginated_sorts_by_amount_both_directions(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let asc = get_paginated_sorted_transactions(
            &pool,
            &1,
            &10,
            &Some("amount".to_owned()),
            &Some(SortDir::Asc),
        )
        .await?;
        let desc = get_paginated_sorted_transactions(
            &pool,
            &1,
            &10,
            &Some("amount".to_owned()),
            &Some(SortDir::Desc),
        )
        .await?;

        assert_eq!(ids(&asc), vec![2, 1, 3, 4]);
        assert_eq!(ids(&desc), vec![4, 3, 1, 2]);
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn paginated_joins_category_details(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let transactions = get_paginated_sorted_transactions(
            &pool,
            &1,
            &1,
            &Some("name".to_owned()),
            &Some(SortDir::Asc),
        )
        .await?;

        let first = &transactions[0];
        // All fixture rows are category_id = 1 ("Uncategorized", no icon).
        assert_eq!(first.category_name, "Uncategorized");
        assert_eq!(first.category_color, "#535353");
        assert_eq!(first.category_icon, None);
        // Deref exposes the underlying Transaction fields.
        assert_eq!(first.name, "TRANSACTION 1");
        Ok(())
    }

    fn category_of(transactions: &[TransactionWithAccount], id: i64) -> &str {
        transactions
            .iter()
            .find(|t| *t.id() == id)
            .map(|t| t.category_name.as_str())
            .expect("transaction should exist")
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn update_reassigns_and_overwrites_category(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        update_transaction_category(&pool, 1, 4, 1).await?;
        assert_eq!(category_of(&all_transactions(&pool).await?, 1), "Groceries");

        // A subsequent update replaces the previous category (last write wins).
        update_transaction_category(&pool, 1, 2, 1).await?;
        assert_eq!(category_of(&all_transactions(&pool).await?, 1), "Income");
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn update_only_affects_target_transaction(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        update_transaction_category(&pool, 1, 4, 1).await?;

        let transactions = all_transactions(&pool).await?;
        assert_eq!(category_of(&transactions, 1), "Groceries");
        for id in [2, 3, 4] {
            assert_eq!(
                category_of(&transactions, id),
                "Uncategorized",
                "transaction {id} should be untouched"
            );
        }
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn update_to_nonexistent_category_is_rejected(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // category_id 999 has no matching category row; the foreign key on
        // transaction.category_id should reject the update.
        let result = update_transaction_category(&pool, 1, 999, 1).await;
        assert!(
            result.is_err(),
            "updating to a non-existent category should fail"
        );

        // The rejected update must leave the transaction's category unchanged.
        assert_eq!(
            category_of(&all_transactions(&pool).await?, 1),
            "Uncategorized"
        );
        Ok(())
    }

    async fn rule_for(pool: &Pool<Sqlite>, name: &str) -> Option<i64> {
        sqlx::query_scalar("SELECT category_id FROM transaction_category_rule WHERE name = ?")
            .bind(name)
            .fetch_optional(pool)
            .await
            .expect("query rule")
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn update_records_and_replaces_merchant_rule(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        update_transaction_category(&pool, 1, 4, 1).await?;
        assert_eq!(rule_for(&pool, "TRANSACTION 1").await, Some(4));

        update_transaction_category(&pool, 1, 2, 1).await?;
        assert_eq!(rule_for(&pool, "TRANSACTION 1").await, Some(2));
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM transaction_category_rule WHERE name = ?")
                .bind("TRANSACTION 1")
                .fetch_one(&pool)
                .await?;
        assert_eq!(count, 1);
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn update_to_uncategorized_clears_merchant_rule(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        update_transaction_category(&pool, 1, 4, 1).await?;
        assert_eq!(rule_for(&pool, "TRANSACTION 1").await, Some(4));

        update_transaction_category(&pool, 1, 1, 1).await?;
        assert_eq!(rule_for(&pool, "TRANSACTION 1").await, None);
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("plaid_sync")))]
    async fn add_applies_saved_merchant_rule(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // A saved rule maps "Coffee" -> Groceries (category 4).
        sqlx::query(
            "INSERT INTO transaction_category_rule (name, category_id) VALUES ('Coffee', 4)",
        )
        .execute(&pool)
        .await?;
        let rules = get_category_rules(&pool).await?;

        let mut conn = pool.acquire().await?;
        add_plaid_transactions(
            &mut conn,
            vec![
                plaid_txn("txn-known", "Coffee", -4.50, false),
                plaid_txn("txn-unknown", "Mystery Shop", -20.00, false),
            ],
            1,
            &rules,
        )
        .await?;

        let category_sql = "SELECT category_id FROM 'transaction' WHERE plaid_transaction_id = ?";
        let known: i64 = sqlx::query_scalar(category_sql)
            .bind("txn-known")
            .fetch_one(&pool)
            .await?;
        let unknown: i64 = sqlx::query_scalar(category_sql)
            .bind("txn-unknown")
            .fetch_one(&pool)
            .await?;

        assert_eq!(known, 4, "rule merchant inherits saved category");
        assert_eq!(unknown, 1, "unknown merchant falls back to default");
        Ok(())
    }
}
