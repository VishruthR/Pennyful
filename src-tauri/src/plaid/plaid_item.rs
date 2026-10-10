use std::fmt;

#[derive(sqlx::FromRow, PartialEq, Debug)]
pub struct PlaidItem {
    item_id: String,
    access_token: String,
    cursor: Option<String>,
}

impl PlaidItem {
    #[allow(dead_code)]
    pub fn item_id(&self) -> &String {
        &self.item_id
    }

    pub fn access_token(&self) -> &String {
        &self.access_token
    }

    pub fn cursor(&self) -> &Option<String> {
        &self.cursor
    }

    #[allow(dead_code)]
    pub fn new(item_id: String, access_token: String, cursor: Option<String>) -> Self {
        PlaidItem {
            item_id,
            access_token,
            cursor,
        }
    }
}

impl fmt::Display for PlaidItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PlaidItem: {} {}",
            self.item_id,
            self.cursor.clone().unwrap_or("No cursor".to_string())
        )
    }
}
