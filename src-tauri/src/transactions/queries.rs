use crate::{plaid::types::PlaidTransaction, types::SortDir};
use crate::types::{Transaction, TransactionWithAccount};
use ::plaid::model::RemovedTransaction;
use sqlx::{Pool, QueryBuilder, Sqlite, SqliteConnection};

pub async fn get_transactions(
    pool: &Pool<Sqlite>,
    limit: Option<i64>,
) -> Result<Vec<Transaction>, sqlx::Error> {
    let query = "SELECT id, plaid_transaction_id, name, merchant_entity_id, amount_cents, date, pending, deleted_at, account_id, category_id FROM 'transaction' WHERE deleted_at IS NULL ORDER BY date, id LIMIT $1";

    // Negative value returns all rows
    let lim = limit.unwrap_or(-1);
    let res: Vec<Transaction> = sqlx::query_as(query).bind(lim).fetch_all(pool).await?;

    Ok(res)
}

pub async fn get_num_transactions(
    pool: &Pool<Sqlite>
) -> Result<i64, sqlx::Error> {
    let res: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM 'transaction' WHERE deleted_at IS NULL")
        .fetch_one(pool)
        .await?;

    Ok(res)
}


pub async fn get_paginated_sorted_transactions(
    pool: &Pool<Sqlite>,
    page: &i64,
    page_size: &i64,
    sort_col: &Option<String>,
    sort_dir: &Option<SortDir>
) -> Result<Vec<TransactionWithAccount>, sqlx::Error> {
    let offset = std::cmp::max(page - 1, 0) * page_size;

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(r#"
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
            c.name AS category_name,
            c.color AS category_color,
            c.icon AS category_icon
        FROM 'transaction' t
        JOIN account a ON t.account_id=a.id
        JOIN category c ON t.category_id=c.id
        WHERE t.deleted_at IS NULL
    "#);

    query_builder.push(" ORDER BY ");
    if let Some(col) = sort_col.as_deref() {
        let sort_col_final = match col {
            "date" => "t.date",
            "account" => "a.name",
            "name" => "t.name",
            "amount" => "t.amount_cents",
            _ => return Err(sqlx::Error::Protocol("Invalid sort column".into()))
        };

        let sort_dir_final = match sort_dir.unwrap_or(SortDir::Asc) {
            SortDir::Asc => "ASC",
            SortDir::Desc => "DESC"
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

    let transactions: Vec<TransactionWithAccount> = query_builder.build_query_as::<TransactionWithAccount>()
        .fetch_all(pool)
        .await?;

    Ok(transactions)
}

pub async fn add_plaid_transactions(
    conn: &mut SqliteConnection,
    new_transactions: Vec<PlaidTransaction>,
    default_category: &i64,
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

        b.push_bind(t.plaid_transaction_id)
            .push_bind(t.name.unwrap_or("".to_string()))
            .push_bind(t.merchant_entity_id)
            .push_bind(t.amount)
            .push_bind(t.date)
            .push_bind(t.pending)
            .push_bind(account_id)
            .push_bind(default_category);
    });
    query_builder.push(
        " ON CONFLICT(plaid_transaction_id) WHERE plaid_transaction_id IS NOT NULL DO NOTHING",
    );

    let query = query_builder.build();
    let res = query.execute(&mut *conn).await?;

    Ok(num_transactions - res.rows_affected())
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
    use crate::types::Cents;
    use chrono::NaiveDate;
    use rust_decimal::dec;

    fn get_expected_transactions() -> Vec<Transaction> {
        vec![
            Transaction::new(
                1,
                "TRANSACTION 1".to_owned(),
                Cents(dec!(-5.77)),
                NaiveDate::from_ymd_opt(2025, 12, 15).unwrap(),
                1,
                1,
            ),
            Transaction::new(
                2,
                "TRANSACTION 2".to_owned(),
                Cents(dec!(-10.90)),
                NaiveDate::from_ymd_opt(2025, 12, 16).unwrap(),
                1,
                1,
            ),
            Transaction::new(
                4,
                "TRANSACTION 4".to_owned(),
                Cents(dec!(-0.70)),
                NaiveDate::from_ymd_opt(2025, 12, 16).unwrap(),
                1,
                1,
            ),
            Transaction::new(
                3,
                "TRANSACTION 3".to_owned(),
                Cents(dec!(-1.90)),
                NaiveDate::from_ymd_opt(2025, 12, 17).unwrap(),
                1,
                1,
            ),
        ]
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn test_get_transactions_all(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let transactions = get_transactions(&pool, None).await?;

        assert_eq!(transactions, get_expected_transactions());
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("transactions")))]
    async fn test_get_transactions_limit(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let transactions = get_transactions(&pool, Some(2)).await?;

        assert_eq!(transactions, get_expected_transactions()[..2]);
        Ok(())
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
            Cents::from_dollars_f64(amount_dollars).unwrap(),
            NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
            pending,
            "plaid-acct-1".to_owned(),
            Some(1),
            None,
        )
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
            &1
        )
        .await?;
        assert_eq!(skipped, 0);

        let transactions = get_transactions(&pool, None).await?;
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
        assert_eq!(coffee.amount, Cents::from_dollars_f64(-4.50).unwrap());
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("plaid_sync")))]
    async fn add_skips_duplicate_plaid_transaction_ids(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = pool.acquire().await?;
        let first =
            add_plaid_transactions(&mut conn, vec![plaid_txn("txn-1", "Coffee", -4.50, false)], &1)
                .await?;
        assert_eq!(first, 0);

        // Re-syncing the same transaction (Plaid resends) must not duplicate it, and
        // the whole batch is reported as skipped.
        let second = add_plaid_transactions(
            &mut conn,
            vec![plaid_txn("txn-1", "Coffee CHANGED", -9.99, false)],
            &1
        )
        .await?;
        assert_eq!(second, 1);

        let transactions = get_transactions(&pool, None).await?;
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
            &1
        )
        .await?;

        let mut posted = plaid_txn("txn-1", "Posted Coffee", -5.25, false);
        posted.date = NaiveDate::from_ymd_opt(2026, 1, 20).unwrap();
        modify_plaid_transactions(&mut conn, vec![posted]).await?;

        let transactions = get_transactions(&pool, None).await?;
        let txn = transactions
            .iter()
            .find(|t| t.plaid_transaction_id().as_deref() == Some("txn-1"))
            .expect("transaction should exist");
        assert_eq!(txn.name, "Posted Coffee");
        assert_eq!(txn.amount, Cents::from_dollars_f64(-5.25).unwrap());
        assert_eq!(txn.date, NaiveDate::from_ymd_opt(2026, 1, 20).unwrap());
        assert!(!txn.pending, "pending should flip to posted");
        Ok(())
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("plaid_sync")))]
    async fn remove_soft_deletes_and_hides_transaction(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = pool.acquire().await?;
        add_plaid_transactions(&mut conn, vec![plaid_txn("txn-1", "Coffee", -4.50, false)], &1).await?;

        remove_plaid_transactions(
            &mut conn,
            vec![RemovedTransaction {
                transaction_id: "txn-1".to_owned(),
                ..Default::default()
            }],
        )
        .await?;

        let transactions = get_transactions(&pool, None).await?;
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
    async fn paginated_walks_pages_without_overlap(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(get_num_transactions(&pool).await?, 4);

        // Sort by amount (unique values) so paging is deterministic.
        // Ascending amounts: id2=-10.90, id1=-5.77, id3=-1.90, id4=-0.70
        let sort_col = Some("amount".to_owned());
        let page1 =
            get_paginated_sorted_transactions(&pool, &1, &2, &sort_col, &Some(SortDir::Asc)).await?;
        let page2 =
            get_paginated_sorted_transactions(&pool, &2, &2, &sort_col, &Some(SortDir::Asc)).await?;
        let page3 =
            get_paginated_sorted_transactions(&pool, &3, &2, &sort_col, &Some(SortDir::Asc)).await?;

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
}
