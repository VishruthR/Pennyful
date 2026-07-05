use crate::AppState;
use plaid::PlaidClient;
use plaid::model::{CountryCode, LinkTokenCreateRequestUser, Products};
use plaid::request::link_token_create::LinkTokenCreateRequired;
use crate::plaid::queries::insert_plaid_item;

#[tauri::command]
pub async fn generate_link_token() -> Result<String, String> {
    let client = PlaidClient::from_env();
    println!("hello");

    let client_name = "Pennyful";
    let country_codes = vec![CountryCode::Us];
    let language = "en";
    let user = LinkTokenCreateRequestUser {
        client_user_id: "test_user_id".to_owned(),
        address: None,
        date_of_birth: None,
        email_address: None,
        email_address_verified_time: None,
        id_number: None,
        legal_name: None,
        name: None,
        phone_number: None,
        phone_number_verified_time: None,
        ssn: None,
    };

    println!("right before link_token_create");

    let link_token_create_resp = client.link_token_create(LinkTokenCreateRequired {
            client_name,
            country_codes,
            language,
            user,
        })
        .products(vec![Products::Transactions])
        .await
        .map_err(|e| format!("link_token_create failed: {e}"))?;

    Ok(link_token_create_resp.link_token)
}

#[tauri::command]
pub async fn generate_access_token(state: tauri::State<'_, AppState>, public_token: String) -> Result<u64, String> {
    let db = &state.db;
    let client = PlaidClient::from_env();

    let response = client.item_public_token_exchange(&public_token).await.unwrap();
    let rows_affected = insert_plaid_item(&db.0, response.item_id, response.access_token)
        .await
        .map_err(|e| e.to_string());

    Ok(rows_affected?)
}

