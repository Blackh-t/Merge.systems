use crate::api::systems::blockchain::verification::gets_owner;
use reqwest;

/// Sending a POST to a verification server, and checks of the owner is a member.
/// # Parameter
/// - key: ETH Wallet private-key
/// # Return
/// - True if user is a member else false.
pub async fn check_key(key: String) -> bool {
    // Control checks before sending a request
    let wallet_key = gets_owner(&key).await;
    if wallet_key.is_empty() {
        return false;
    }

    // Defined EndPoint for verification.
    let url = "https://www.merge.systems/.netlify/functions/blockchain";
    let client = reqwest::Client::new();

    // Send the wallet key to the verification.
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(wallet_key)
        .send()
        .await;

    // Check the response from HTTP-server, if wallet key doesn't match return false.
    if let Ok(body) = response {
        if let Ok(text) = body.text().await {
            return text == "pass";
        }
    }
    false
}
