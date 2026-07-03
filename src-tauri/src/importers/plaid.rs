use std::time::Duration;
use plaid::PlaidClient;
use plaid::model::{Products, Transaction, TransactionsUpdateStatus};

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

    let response = client
        .accounts_get(&exchange.access_token)
        .await
        .map_err(|e| format!("accounts_get failed: {e}"))?;

    let account_ids: Vec<String> = response.accounts.into_iter().map(|a| a.account_id).collect();
    println!("{:?}", account_ids);

    let (txns, cursor) = fetch_all_transactions(&client, &exchange.access_token).await.unwrap();
    
    Ok(txns.len().to_string())
}

pub async fn fetch_all_transactions(
    client: &PlaidClient,
    access_token: &str,
) -> Result<(Vec<Transaction>, String), String> {
    let mut all = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let mut req = client.transactions_sync(access_token).count(500);
        if let Some(c) = &cursor {
            req = req.cursor(c);
        }

        let resp = req
            .await
            .map_err(|e| format!("transactions_sync failed: {e}"))?;

        if matches!(resp.transactions_update_status, TransactionsUpdateStatus::NotReady) {
            tokio::time::sleep(Duration::from_secs(2)).await;
            continue;
        }

        all.extend(resp.added);
        cursor = Some(resp.next_cursor);

        if !resp.has_more {
            break;
        }
    }


    Ok((all, cursor.unwrap_or_default()))
}
