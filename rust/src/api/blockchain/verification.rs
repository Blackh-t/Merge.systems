use ethers::prelude::*;

/// Gets Wallet address with their ETH private-key
async fn fetch_wallet_address(priv_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let wallet: LocalWallet = priv_key.parse()?;
    let address = wallet.address();
    Ok(format!("{:?}", address))
}

pub async fn gets_owner(priv_key: &str) -> String {
    // Ignore the error msg to be read by users.
    match fetch_wallet_address(priv_key).await {
        Ok(key) => key,
        _ => String::new(),
    }
}
