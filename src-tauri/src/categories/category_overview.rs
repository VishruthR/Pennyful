use crate::transactions::dollars::Dollars;

#[derive(sqlx::FromRow, PartialEq, Debug, Clone, serde::Serialize)]
pub struct FullCategory {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
    #[sqlx(rename = "budget_cents")]
    pub budget: Option<Dollars>,
    #[sqlx(rename = "spent_cents")]
    pub spent: Dollars,
}
