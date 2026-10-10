use std::fmt;

#[derive(sqlx::FromRow, PartialEq, Debug, serde::Serialize)]
pub struct Bank {
    id: i64,
    plaid_item_id: Option<String>,
    plaid_institution_id: Option<String>,
    bank_name: String,
}

impl Bank {
    #[allow(dead_code)]
    pub fn new(
        id: i64,
        plaid_item_id: Option<String>,
        plaid_institution_id: Option<String>,
        bank_name: String,
    ) -> Self {
        Bank {
            id,
            plaid_item_id,
            plaid_institution_id,
            bank_name,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    #[allow(dead_code)]
    pub fn plaid_item_id(&self) -> &Option<String> {
        &self.plaid_item_id
    }
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bank: {} {} {} {}",
            self.id,
            self.plaid_item_id
                .clone()
                .unwrap_or("no_item_id".to_string()),
            self.plaid_institution_id
                .clone()
                .unwrap_or("no_institution_id".to_string()),
            self.bank_name
        )
    }
}
