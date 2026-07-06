use crate::AppState;
use plaid::{PlaidClient, model::{CountryCode, HostedLinkDeliveryMethod, LinkTokenCreateHostedLink, LinkTokenCreateRequestUser, Products}, request::link_token_create::LinkTokenCreateRequired};
use crate::plaid::queries::insert_plaid_item;

#[tauri::command]
pub async fn generate_link_token(state: tauri::State<'_, AppState>, phone_number: String) -> Result<String, String> {
    let client = PlaidClient::from_env();

    println!("Hello");
    println!("Phone number: {:?}", phone_number);

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

    println!("{:?}", link_token_create_resp);

    let mut link_token = state.link_token.lock().await;
    *link_token = Some(link_token_create_resp.link_token.clone());
    Ok(link_token_create_resp.hosted_link_url.ok_or("")?)
}

#[tauri::command]
pub async fn generate_access_token_from_hosted_link(state: tauri::State<'_, AppState>) -> Result<u64, String> {
    let db = &state.db;
    let client = PlaidClient::from_env();

    let link_token = state.link_token.lock().await;
    let Some(ref link_token) = *link_token else {
        return Err("Link token not set".to_owned());
    };

    let link_token_resp = client.link_token_get(&link_token)
        .await
        .unwrap();
    
    println!("Link token: {:?}", link_token_resp.link_token);

    let public_token = link_token_resp
        .link_sessions
        .unwrap()
        .into_iter()
        .filter_map(|s| s.results)
        .flat_map(|r| r.item_add_results)
        .map(|item| item.public_token)
        .next()
        .ok_or("no public_token in link sessions yet")?;

    println!("public token: {:?}", public_token);

    let response = client.item_public_token_exchange(&public_token).await.unwrap();
    println!("response: {:#?}", response);

    let rows_affected = insert_plaid_item(&db.0, response.item_id, response.access_token)
        .await
        .map_err(|e| e.to_string());

    Ok(rows_affected?)
}

