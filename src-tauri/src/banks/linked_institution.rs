/// A previously-linked institution surfaced to the frontend so the user can add
/// more of its accounts without re-linking through Plaid Link.
#[derive(sqlx::FromRow, serde::Serialize, Debug, PartialEq)]
pub struct LinkedInstitution {
    pub item_id: String,
    pub institution_name: String,
    pub institution_id: Option<String>,
    pub account_count: i64,
}
