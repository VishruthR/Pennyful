#[derive(sqlx::FromRow, Eq, PartialEq, Debug, Clone, serde::Serialize)]
pub struct Category {
    id: i64,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
}

impl Category {
    pub fn id(&self) -> &i64 {
        &self.id
    }
}
