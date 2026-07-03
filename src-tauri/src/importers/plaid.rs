use plaid::PlaidClient;
use plaid::model::Products;

/// Sandbox test flow: mints a public token for a test institution, then
/// exchanges it for a durable access token. Returns the access token so we can
/// confirm the Plaid integration works end-to-end without the Link UI.
#[tauri::command]
pub async fn connect_to_plaid() -> Result<String, String> {
    let client = PlaidClient::from_env();

    // "ins_109508" is Plaid's sandbox test institution (First Platypus Bank).
    let public_token = client
        .sandbox_public_token_create(vec![Products::Transactions], "ins_109508")
        .await
        .map_err(|e| format!("sandbox_public_token_create failed: {e}"))?
        .public_token;

    let exchange = client
        .item_public_token_exchange(&public_token)
        .await
        .map_err(|e| format!("item_public_token_exchange failed: {e}"))?;

    println!("Connected item {} -> access token {}", exchange.item_id, exchange.access_token);

    Ok(exchange.access_token)
}
