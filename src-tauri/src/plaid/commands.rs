use crate::AppState;
use plaid::{PlaidAuth, PlaidClient, model::{CountryCode, LinkTokenCreateHostedLink, LinkTokenCreateRequestUser, Products}, request::link_token_create::LinkTokenCreateRequired};
use crate::plaid::queries::insert_plaid_item;
use dotenvy_macro::dotenv;
use httpclient::Client;

#[tauri::command]
pub async fn generate_link_token(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let http_client = Client::new().base_url(dotenv!("PLAID_ENV"));
    let auth = PlaidAuth::ClientId { 
        client_id: dotenv!("PLAID_CLIENT_ID").to_string(),
        secret: dotenv!("PLAID_SECRET").to_string(),
        version: dotenv!("PLAID_VERSION").to_string()
    };
    let client = PlaidClient::new(http_client, auth);

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

    let link_token_create_resp = client.link_token_create(LinkTokenCreateRequired {
            client_name,
            country_codes,
            language,
            user,
        })
        .hosted_link(LinkTokenCreateHostedLink {
            completion_redirect_uri: Some("pennyful://plaid-complete".to_owned()),
            delivery_method: None,
            is_mobile_app: Some(false),
            url_lifetime_seconds: Some(300),
        })
        .products(vec![Products::Transactions])
        .await
        .map_err(|e| format!("link_token_create failed: {e}"))?;

    let mut link_token = state.link_token.lock().await;
    *link_token = Some(link_token_create_resp.link_token.clone());
    Ok(link_token_create_resp.hosted_link_url.ok_or("no hosted_link_url returned")?)
}

#[tauri::command]
pub async fn generate_access_token_from_hosted_link(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let db = &state.db;
    let http_client = Client::new().base_url(dotenv!("PLAID_ENV"));
    let auth = PlaidAuth::ClientId { 
        client_id: dotenv!("PLAID_CLIENT_ID").to_string(),
        secret: dotenv!("PLAID_SECRET").to_string(),
        version: dotenv!("PLAID_VERSION").to_string()
    };
    let client = PlaidClient::new(http_client, auth);


    let link_token = state.link_token.lock().await;
    let Some(ref link_token) = *link_token else {
        return Err("Link token not set".to_owned());
    };

    let link_token_resp = client.link_token_get(&link_token)
        .await
        .map_err(|e| format!("link_token_get failed: {e}"))?;
    
    let public_token = link_token_resp
        .link_sessions
        .unwrap_or_default()
        .into_iter()
        .filter_map(|s| s.results)
        .flat_map(|r| r.item_add_results)
        .map(|item| item.public_token)
        .next()
        .ok_or("no public_token in link sessions yet")?;

    let response = client.item_public_token_exchange(&public_token).await.map_err(|e| format!("token exchange failed: {e}"))?;

    let _rows_affected = insert_plaid_item(&db.0, response.item_id, &response.access_token)
        .await
        .map_err(|e| e.to_string());

    Ok(response.access_token)
}

