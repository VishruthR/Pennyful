use crate::types::{Category, CategoryOverview};
use sqlx::{Pool, Sqlite};

pub async fn get_all_categories(pool: &Pool<Sqlite>) -> Result<Vec<Category>, sqlx::Error> {
    let query = "SELECT id, name, color, icon FROM category ORDER BY id";

    let res: Vec<Category> = sqlx::query_as(query).fetch_all(pool).await?;

    Ok(res)
}

/// Returns every category with its budget (if any) and the amount spent in the
/// current calendar month. Spending is stored as negative cents, so the net of all
/// transactions is negated into a positive "spent" total; income/refunds (positive
/// amounts) are included and reduce the total toward (or below) zero.
pub async fn get_category_overviews(
    pool: &Pool<Sqlite>,
) -> Result<Vec<CategoryOverview>, sqlx::Error> {
    let query = r#"
        SELECT
            c.id AS id,
            c.name AS name,
            c.color AS color,
            c.icon AS icon,
            b.amount_cents AS budget_cents,
            COALESCE((
                SELECT -SUM(t.amount_cents)
                FROM "transaction" t
                WHERE t.category_id = c.id
                    AND t.deleted_at IS NULL
                    AND strftime('%Y-%m', t.date) = strftime('%Y-%m', 'now')
            ), 0) AS spent_cents
        FROM category c
        LEFT JOIN budget b ON b.category_id = c.id
        ORDER BY c.id
    "#;

    let res: Vec<CategoryOverview> = sqlx::query_as(query).fetch_all(pool).await?;

    Ok(res)
}

/// Inserts or updates the budget for a category.
pub async fn upsert_budget(
    pool: &Pool<Sqlite>,
    category_id: i64,
    amount_cents: i64,
) -> Result<(), sqlx::Error> {
    let query = r#"
        INSERT INTO budget (category_id, amount_cents)
        VALUES (?, ?)
        ON CONFLICT(category_id) DO UPDATE SET
            amount_cents = excluded.amount_cents,
            updated_at = datetime('now')
    "#;

    sqlx::query(query)
        .bind(category_id)
        .bind(amount_cents)
        .execute(pool)
        .await?;

    Ok(())
}

/// Creates a category and, if a budget is provided, sets its budget.
/// Returns the id of the new category.
pub async fn create_category(
    pool: &Pool<Sqlite>,
    name: &str,
    color: &str,
    icon: Option<&str>,
    budget_cents: Option<i64>,
) -> Result<i64, sqlx::Error> {
    let id: i64 =
        sqlx::query_scalar("INSERT INTO category (name, color, icon) VALUES (?, ?, ?) RETURNING id")
            .bind(name)
            .bind(color)
            .bind(icon)
            .fetch_one(pool)
            .await?;

    if let Some(cents) = budget_cents {
        upsert_budget(pool, id, cents).await?;
    }

    Ok(id)
}

/// Updates a category's name/color/icon and, if a budget is provided, its budget.
pub async fn update_category(
    pool: &Pool<Sqlite>,
    id: i64,
    name: &str,
    color: &str,
    icon: Option<&str>,
    budget_cents: Option<i64>,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE category SET name = ?, color = ?, icon = ? WHERE id = ?")
        .bind(name)
        .bind(color)
        .bind(icon)
        .bind(id)
        .execute(pool)
        .await?;

    if let Some(cents) = budget_cents {
        upsert_budget(pool, id, cents).await?;
    }

    Ok(())
}

/// Deletes a category. Any transactions in that category are first reassigned to
/// "Uncategorized" so no spending is lost. The budget row is removed by the
/// ON DELETE CASCADE foreign key.
pub async fn delete_category(pool: &Pool<Sqlite>, id: i64) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let uncategorized_id: i64 =
        sqlx::query_scalar("SELECT id FROM category WHERE name = 'Uncategorized'")
            .fetch_one(&mut *tx)
            .await?;

    sqlx::query("UPDATE \"transaction\" SET category_id = ? WHERE category_id = ?")
        .bind(uncategorized_id)
        .bind(id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM category WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_expected_categories() -> Vec<String> {
        // Only test default categories
        return vec![
            "Uncategorized".to_string(),
            "Income".to_string(),
            "Housing".to_string(),
            "Groceries".to_string(),
            "Restaurants".to_string(),
            "Transportation".to_string(),
            "Healthcare".to_string(),
            "Savings".to_string(),
            "Education".to_string(),
            "Entertainment".to_string(),
            "Shopping".to_string(),
            "Hobbies".to_string(),
            "Miscellaneous".to_string(),
        ];
    }

    async fn test_get_all_transactions(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let categories = get_all_categories(&pool).await?;

        let category_names: Vec<String> = categories.iter().map(|c| c.name.clone()).collect();

        assert_eq!(category_names, get_expected_categories());
        Ok(())
    }

    /// Seeds a single bank + account so transactions can satisfy their FK.
    async fn seed_account(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO bank (id, bank_name) VALUES (1, 'Test Bank')")
            .execute(pool)
            .await?;
        sqlx::query(
            "INSERT INTO account (id, name, bank_id, account_type) VALUES (1, 'Acct', 1, 'CHECKINGS')",
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn insert_txn(
        pool: &Pool<Sqlite>,
        category_id: i64,
        amount_cents: i64,
        date: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO \"transaction\" (name, amount_cents, date, account_id, category_id) VALUES ('t', ?, ?, 1, ?)",
        )
        .bind(amount_cents)
        .bind(date)
        .bind(category_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    fn overview_for<'a>(
        overviews: &'a [CategoryOverview],
        id: i64,
    ) -> &'a CategoryOverview {
        overviews
            .iter()
            .find(|o| o.id == id)
            .expect("category should be present in overviews")
    }

    #[sqlx::test]
    async fn test_overviews_include_all_categories_with_null_budget(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let overviews = get_category_overviews(&pool).await?;

        // Every seeded category is present, including Uncategorized (id 1).
        assert_eq!(overviews.len(), get_expected_categories().len());
        assert!(overviews.iter().any(|o| o.name == "Uncategorized"));

        // With no budget row and no transactions, budget is null and spend is 0.
        let uncategorized = overview_for(&overviews, 1);
        assert_eq!(uncategorized.budget_cents, None);
        assert_eq!(uncategorized.spent_cents, 0);
        Ok(())
    }

    #[sqlx::test]
    async fn test_overviews_current_month_spend_nets_refunds(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        seed_account(&pool).await?;
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();

        // Category 3 (Housing): a spend and a refund this month net to 700 cents spent.
        insert_txn(&pool, 3, -1000, &today).await?;
        insert_txn(&pool, 3, 300, &today).await?;
        // A prior-month transaction is excluded from the current-month total.
        insert_txn(&pool, 3, -5000, "2020-01-15").await?;

        let overviews = get_category_overviews(&pool).await?;
        assert_eq!(overview_for(&overviews, 3).spent_cents, 700);
        Ok(())
    }

    #[sqlx::test]
    async fn test_upsert_budget_inserts_then_updates(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        upsert_budget(&pool, 3, 50000).await?;
        assert_eq!(overview_for(&get_category_overviews(&pool).await?, 3).budget_cents, Some(50000));

        upsert_budget(&pool, 3, 12500).await?;
        assert_eq!(overview_for(&get_category_overviews(&pool).await?, 3).budget_cents, Some(12500));
        Ok(())
    }

    #[sqlx::test]
    async fn test_create_category_with_budget(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let id = create_category(&pool, "Coffee", "#6F4E37", Some("mdi:coffee"), Some(2000)).await?;

        let overviews = get_category_overviews(&pool).await?;
        let created = overview_for(&overviews, id);
        assert_eq!(created.name, "Coffee");
        assert_eq!(created.color, "#6F4E37");
        assert_eq!(created.icon.as_deref(), Some("mdi:coffee"));
        assert_eq!(created.budget_cents, Some(2000));
        Ok(())
    }

    #[sqlx::test]
    async fn test_update_category_changes_fields_and_budget(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Category 3 (Housing) starts with no budget.
        update_category(&pool, 3, "Home", "#000000", Some("mdi:home"), Some(30000)).await?;

        let overviews = get_category_overviews(&pool).await?;
        let updated = overview_for(&overviews, 3);
        assert_eq!(updated.name, "Home");
        assert_eq!(updated.color, "#000000");
        assert_eq!(updated.icon.as_deref(), Some("mdi:home"));
        assert_eq!(updated.budget_cents, Some(30000));
        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_category_reassigns_transactions_to_uncategorized(
        pool: Pool<Sqlite>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        seed_account(&pool).await?;
        // A transaction in category 3 (Housing), plus a budget for it.
        insert_txn(&pool, 3, -1000, "2020-01-15").await?;
        upsert_budget(&pool, 3, 50000).await?;

        delete_category(&pool, 3).await?;

        // The category is gone.
        let overviews = get_category_overviews(&pool).await?;
        assert!(overviews.iter().all(|o| o.id != 3));

        // Its transaction now belongs to Uncategorized (id 1); none remain on category 3.
        let uncategorized_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM \"transaction\" WHERE category_id = 1")
                .fetch_one(&pool)
                .await?;
        assert_eq!(uncategorized_count, 1);
        let orphan_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM \"transaction\" WHERE category_id = 3")
                .fetch_one(&pool)
                .await?;
        assert_eq!(orphan_count, 0);

        // The budget row was cascaded away.
        let budget_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM budget WHERE category_id = 3")
                .fetch_one(&pool)
                .await?;
        assert_eq!(budget_count, 0);
        Ok(())
    }
}
