#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(rename_all = "UPPERCASE")]
pub enum SortDir {
    Asc,
    Desc,
}
