#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, serde::Serialize)]
#[sqlx(rename_all = "UPPERCASE")]
pub enum AccountType {
    Savings,
    Checkings,
    Credit,
}
