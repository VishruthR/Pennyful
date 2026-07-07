use crate::AppState;
use plaid::{PlaidAuth, PlaidClient, model::{CountryCode, LinkTokenCreateHostedLink, LinkTokenCreateRequestUser, LinkTokenGetSessionsResponse, Products}, request::link_token_create::LinkTokenCreateRequired};
use crate::plaid::queries::insert_plaid_item;
use dotenvy_macro::dotenv;
use httpclient::Client;

// completion_redirect_uri is where Plaid sends the browser once the Hosted Link
// flow finishes. Under `tauri dev` the pennyful:// deep link can't reach the
// running dev binary (macOS routes custom schemes only to a built .app bundle),
// so we omit it and finish via the manual button; real builds use the deep link.
fn completion_redirect_uri_for(is_dev: bool) -> Option<String> {
    if is_dev {
        None
    } else {
        Some("pennyful://plaid-complete".to_owned())
    }
}

// Pull the first public_token out of a completed Hosted Link session. Returns an
// error while the flow is still unfinished (no session results yet).
fn extract_public_token(
    link_sessions: Option<Vec<LinkTokenGetSessionsResponse>>,
) -> Result<String, String> {
    link_sessions
        .unwrap_or_default()
        .into_iter()
        .filter_map(|s| s.results)
        .flat_map(|r| r.item_add_results)
        .map(|item| item.public_token)
        .next()
        .ok_or_else(|| "no public_token in link sessions yet".to_owned())
}

fn plaid_client() -> PlaidClient {
    let http_client = Client::new().base_url(dotenv!("PLAID_ENV"));
    let auth = PlaidAuth::ClientId {
        client_id: dotenv!("PLAID_CLIENT_ID").to_string(),
        secret: dotenv!("PLAID_SECRET").to_string(),
        version: dotenv!("PLAID_VERSION").to_string(),
    };
    PlaidClient::new(http_client, auth)
}

#[tauri::command]
pub async fn generate_link_token(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let client = plaid_client();
    let completion_redirect_uri = completion_redirect_uri_for(tauri::is_dev());

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
            completion_redirect_uri,
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

pub async fn complete_hosted_link(state: &AppState) -> Result<u64, String> {
    let db = &state.db;
    let client = plaid_client();

    let link_token = state.link_token.lock().await;
    let Some(ref link_token) = *link_token else {
        return Err("Link token not set".to_owned());
    };

    let link_token_resp = client.link_token_get(&link_token)
        .await
        .map_err(|e| format!("link_token_get failed: {e}"))?;

    let public_token = extract_public_token(link_token_resp.link_sessions)?;

    let response = client.item_public_token_exchange(&public_token).await.map_err(|e| format!("token exchange failed: {e}"))?;

    let rows_affected = insert_plaid_item(&db.0, response.item_id, &response.access_token)
        .await
        .map_err(|e| e.to_string());

    Ok(rows_affected?)
}

#[tauri::command]
pub async fn generate_access_token_from_hosted_link(state: tauri::State<'_, AppState>) -> Result<u64, String> {
    complete_hosted_link(state.inner()).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn dev_build_omits_completion_redirect() {
        // In dev we can't receive the deep link, so Plaid should not redirect.
        assert_eq!(completion_redirect_uri_for(true), None);
    }

    #[test]
    fn real_build_redirects_to_deep_link() {
        assert_eq!(
            completion_redirect_uri_for(false),
            Some("pennyful://plaid-complete".to_owned())
        );
    }

    fn sessions(value: serde_json::Value) -> Option<Vec<LinkTokenGetSessionsResponse>> {
        Some(serde_json::from_value(value).expect("valid session fixture"))
    }

    #[test]
    fn extracts_public_token_from_finished_session() {
        let s = sessions(json!([
            {
                "link_session_id": "sess-1",
                "results": { "item_add_results": [ { "public_token": "public-123" } ] }
            }
        ]));
        assert_eq!(extract_public_token(s), Ok("public-123".to_owned()));
    }

    #[test]
    fn returns_first_public_token_when_session_added_multiple_items() {
        let s = sessions(json!([
            {
                "link_session_id": "sess-1",
                "results": { "item_add_results": [
                    { "public_token": "first" },
                    { "public_token": "second" }
                ] }
            }
        ]));
        assert_eq!(extract_public_token(s), Ok("first".to_owned()));
    }

    #[test]
    fn errors_when_session_started_but_not_finished() {
        // User opened Link (a session exists) but hasn't completed it yet, so
        // there are no results and no public_token to hand back.
        let s = sessions(json!([ { "link_session_id": "sess-1" } ]));
        assert!(extract_public_token(s).is_err());
    }

    #[test]
    fn errors_when_no_sessions_present() {
        assert!(extract_public_token(None).is_err());
        assert!(extract_public_token(Some(vec![])).is_err());
    }
}
