use super::trade_order::*;
use super::{super::super::systems::encoder::hmac, trade_order};
use crate::api::systems::encoder::hmac::{encrypt_pass, encrypt_prehash};
use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue, CONTENT_TYPE};
use std::{
    time::{SystemTime, UNIX_EPOCH},
    u16,
};

/// Used Original data to build headers with HMAC-SHA256.
#[derive(Clone)]
pub struct KuCoinClient {
    pub api_key: String,
    pub api_secret: String,
    pub api_passphrase: String,
    pub timestamp: String,
    pub base_link: String,
    pub endpoint: String,
}

impl KuCoinClient {
    /// Build an new client with generated timestamp.
    ///
    /// # Parameters
    /// - api_key   : Is used in Headers.
    /// - api_secret: Is used to generate base64 encode.
    /// - base_link : Host.
    /// - endpoint  : Path.
    ///
    /// # Returns
    /// - specified 'KuCoinClient' with generated timestamp as milli secound.
    fn build(
        api_key: String,
        api_secret: String,
        api_passphrase: String,
        base_link: String,
        endpoint: String,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_millis()
            .to_string();
        KuCoinClient {
            api_key,
            api_secret,
            api_passphrase,
            timestamp,
            base_link,
            endpoint,
        }
    }

    /// Build headers with generated encoded for KC-API-SIGN and KC-API-PASSPHRASE.
    ///
    /// # Parameters
    /// - payload   : Body for HTTP-request.
    /// - method    : HTTP-request method.
    ///
    /// # Returns
    /// - If headers value passed : A set of HTTP headers.
    /// - If not passed: Error msg.
    fn generate_headers(
        self,
        payload: String,
        method: String,
    ) -> Result<HeaderMap, InvalidHeaderValue> {
        // Encrypting
        let sign = encrypt_prehash(
            self.api_secret.clone(),
            self.timestamp.clone(),
            method,
            self.endpoint,
            payload,
        );
        let passphrase = encrypt_pass(self.api_secret, self.api_passphrase);

        // Build Headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("KC-API-KEY", HeaderValue::from_str(self.api_key.as_str())?);
        headers.insert("KC-API-SIGN", HeaderValue::from_str(sign.as_str())?);
        headers.insert(
            "KC-API-TIMESTAMP",
            HeaderValue::from_str(self.timestamp.as_str())?,
        );
        headers.insert(
            "KC-API-PASSPHRASE",
            HeaderValue::from_str(passphrase.as_str())?,
        );
        headers.insert("KC-API-KEY-VERSION", HeaderValue::from_static("3"));
        Ok(headers)
    }

    /// Placing a long position on Market Price with tp,sl, and leverage.
    ///
    /// # Parameters
    /// - symbol    : Trade symbol.
    /// - usdt      : Number of contract in lot.
    /// - tp_persent: Percent of take profit.
    /// - sl_persent: Percent of stop loss.
    /// - leverage  : Leverage for the trade.
    ///
    /// # Returns
    /// - HTTP-response massages.
    async fn open_long(
        self,
        symbol: &str,
        usdt: u16,
        tp_persent: u16,
        sl_persent: u16,
        leverage: &str,
    ) -> String {
        // Build HTTP-request Body as JSON-String.
        let payload = TradeOrder::create_long_position_payload(
            symbol, usdt, tp_persent, sl_persent, leverage,
        )
        .await
        .serialize()
        .unwrap();

        // Build HTTP-request Headers.
        let headers = self
            .clone()
            .generate_headers(payload.clone(), "POST".to_string());

        // Create a new client then gets the response from HTTP-request.
        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "{}{}",
                self.base_link.clone(),
                self.endpoint.clone()
            ))
            .headers(headers.unwrap())
            .body(payload)
            .send()
            .await
            .unwrap();

        // Returning the result.
        let status = response.status();
        let text = response.text().await;
        format!("{}::{:?}", status, text)
    }

    /// Placing a short position on Market Price with tp,sl, and leverage.
    ///
    /// # Parameters
    /// - symbol    : Trade symbol.
    /// - usdt      : Number of contract in lot.
    /// - tp_persent: Percent of take profit.
    /// - sl_persent: Percent of stop loss.
    /// - leverage  : Leverage for the trade.
    ///
    /// # Returns
    /// - HTTP-response massages.
    async fn open_short(
        self,
        symbol: &str,
        usdt: u16,
        tp_persent: u16,
        sl_persent: u16,
        leverage: &str,
    ) -> String {
        // Build HTTP-request Body as JSON-String.
        let payload = TradeOrder::create_short_position_payload(
            symbol, usdt, tp_persent, sl_persent, leverage,
        )
        .await
        .serialize()
        .unwrap();

        // Build HTTP-request Headers.
        let headers = self
            .clone()
            .generate_headers(payload.clone(), "POST".to_string());

        // Create a new client then gets the response from HTTP-request.
        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "{}{}",
                self.base_link.clone(),
                self.endpoint.clone()
            ))
            .headers(headers.unwrap())
            .body(payload)
            .send()
            .await
            .unwrap();

        let status = response.status();
        let text = response.text().await;
        format!("{}::{:?}", status, text)
    }

    /// Sending a signal to close all of the position on Market based on trade symbol.
    ///
    /// # Parameters
    /// - symbol    : Trade symbol.
    /// - usdt      : _dead_param.
    ///
    /// # Returns
    /// - HTTP-response massages.
    async fn close_orders(self, symbol: &str, usdt: u16) -> String {
        let payload = TradeOrder::create_close_position_payload(symbol, usdt)
            .serialize()
            .unwrap();

        let headers = self
            .clone()
            .generate_headers(payload.clone(), "POST".to_string());
        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "{}{}",
                self.base_link.clone(),
                self.endpoint.clone()
            ))
            .headers(headers.unwrap())
            .body(payload)
            .send()
            .await
            .unwrap();

        // Returning the result.
        let status = response.status();
        let text = response.text().await;
        format!("{}::{:?}", status, text)
    }
}

/// Enter Long position, if there is an position then add a new position, else close the Short then
/// open a new Long.
///
/// # Returns
/// - The HTTP-response.
pub async fn enter_long(
    api_key: String,
    api_secret: String,
    api_passphrase: String,
    base_link: String,
    endpoint: String,
    symbol: &str,
    usdt: u16,
    tp_persent: u16,
    sl_persent: u16,
    leverage: &str,
    last_traded: String,
) -> String {
    // Buuld a new client.
    let client = KuCoinClient::build(api_key, api_secret, api_passphrase, base_link, endpoint);

    // Add position if signal is not short, else Swap to Short.
    if last_traded.as_str() != "Short" {
        let response_open = client
            .open_long(symbol, usdt, tp_persent, sl_persent, leverage)
            .await;
        format!("{}", response_open)
    } else {
        let response_close = client.clone().close_orders(symbol, usdt).await;
        let response_open = client
            .open_long(symbol, usdt, tp_persent, sl_persent, leverage)
            .await;
        format!("{}\n{}", response_close, response_open)
    }
}

/// Enter Short position, if there is an position then add a new position, else close the Long then
/// open a new Short.
///
/// # Returns
/// - The HTTP-response.
pub async fn enter_short(
    api_key: String,
    api_secret: String,
    api_passphrase: String,
    base_link: String,
    endpoint: String,
    symbol: &str,
    usdt: u16,
    tp_persent: u16,
    sl_persent: u16,
    leverage: &str,
    last_traded: String,
) -> String {
    let client = KuCoinClient::build(api_key, api_secret, api_passphrase, base_link, endpoint);
    if last_traded.as_str() != "Long" {
        let response_open = client
            .open_short(symbol, usdt, tp_persent, sl_persent, leverage)
            .await;
        format!("{}", response_open)
    } else {
        let response_close = client.clone().close_orders(symbol, usdt).await;
        let response_open = client
            .open_short(symbol, usdt, tp_persent, sl_persent, leverage)
            .await;
        format!("{}\n{}", response_close, response_open)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_place_short_long() {
        let api_key = env::var("api_key").unwrap();
        let api_secret = env::var("api_secret").unwrap();
        let api_passphrase = env::var("api_passphrase").unwrap();
        let base_link = "https://api-futures.kucoin.com".to_string();
        let endpoint = "/api/v1/st-orders".to_string();

        let response = enter_short(
            api_key,
            api_secret,
            api_passphrase,
            base_link,
            endpoint,
            "XRP",
            2,
            15,
            12,
            "70",
            "Long".to_string(),
        )
        .await;

        println!("{}", response)
    }
}
