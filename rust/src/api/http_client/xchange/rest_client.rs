use super::super::super::systems::encoder::hmac;
use super::trade_order::*;
use crate::api::systems::encoder::hmac::{encrypt_pass, encrypt_prehash};
use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue, CONTENT_TYPE};
use std::{
    time::{SystemTime, UNIX_EPOCH},
    u16,
};

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

    fn generate_headers(
        self,
        payload: String,
        method: String,
    ) -> Result<HeaderMap, InvalidHeaderValue> {
        // Generate Encode.
        let sign = encrypt_prehash(
            self.api_secret.clone(),
            self.timestamp.clone(),
            method,
            self.endpoint,
            payload,
        );
        let passphrase = encrypt_pass(self.api_secret, self.api_passphrase);

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

    async fn open_long(
        self,
        symbol: &str,
        usdt: u16,
        tp_persent: u16,
        sl_persent: u16,
        leverage: &str,
    ) -> String {
        let payload = TradeOrder::create_long_position_payload(
            symbol, usdt, tp_persent, sl_persent, leverage,
        )
        .await
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

        let status = response.status();
        let text = response.text().await;
        format!("{}::{:?}", status, text)
    }

    async fn open_short(
        self,
        symbol: &str,
        usdt: u16,
        tp_persent: u16,
        sl_persent: u16,
        leverage: &str,
    ) -> String {
        let payload = TradeOrder::create_short_position_payload(
            symbol, usdt, tp_persent, sl_persent, leverage,
        )
        .await
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

        let status = response.status();
        let text = response.text().await;
        format!("{}::{:?}", status, text)
    }
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

        let status = response.status();
        let text = response.text().await;
        format!("{}::{:?}", status, text)
    }
}

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
) -> String {
    let client = KuCoinClient::build(api_key, api_secret, api_passphrase, base_link, endpoint);
    let response_close = client.clone().close_orders(symbol, usdt).await;
    let response_open = client
        .open_long(symbol, usdt, tp_persent, sl_persent, leverage)
        .await;
    format!("{}\n{}", response_close, response_open)
}

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
) -> String {
    let client = KuCoinClient::build(api_key, api_secret, api_passphrase, base_link, endpoint);
    let response_close = client.clone().close_orders(symbol, usdt).await;
    let response_open = client
        .open_short(symbol, usdt, tp_persent, sl_persent, leverage)
        .await;
    format!("{}\n{}", response_close, response_open)
}

#[cfg(test)]
mod test {

    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_place_order_long() {
        let api_key = env::var("api_key").unwrap();
        let api_secret = env::var("api_secret").unwrap();
        let api_passphrase = env::var("api_passphrase").unwrap();
        let base_link = "https://api-futures.kucoin.com".to_string();
        let endpoint = "/api/v1/orders".to_string();

        let response = enter_long(
            api_key,
            api_secret,
            api_passphrase,
            base_link,
            endpoint,
            "ZEN",
            1,
            20,
            12,
            "50",
        )
        .await;

        println!("{}", response)
    }
}
