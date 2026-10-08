use crate::transactions::cents::Cents;

#[derive(sqlx::FromRow, PartialEq, Debug, Clone, serde::Serialize)]
pub struct CategoryOverview {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
    #[sqlx(rename = "budget_cents")]
    pub budget: Option<Cents>,
    #[sqlx(rename = "spent_cents")]
    pub spent: Cents,
}
