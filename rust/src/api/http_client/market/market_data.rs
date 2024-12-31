use super::super::super::systems::market::core::*;
use core::f64;
use reqwest;
use std::error::Error;

impl MarketData {
    /// Sending a HTTP request to binance then extracting the closed prices into a list.
    ///
    /// # Parameters
    /// - symbol: Ticket ID
    /// - interval: time interval per candlestick [1m, 5m, 15m ...]
    /// - limit: Number of candlesticks per fetch.
    ///
    /// # Return
    /// - A list of candlesticks closed price.
    async fn market_data(
        symbol: &str,
        interval: &str,
        limit: u8,
    ) -> Result<Vec<DataPoint>, Box<dyn Error>> {
        // Build the EndPoint link.
        let endpoint = format!(
            "https://api.binance.com/api/v1/klines?symbol={}USDT&interval={}&limit={}",
            symbol, interval, limit
        );

        // Send the request.
        let client = reqwest::Client::new();
        let response = client.get(endpoint).send().await?;

        // parsed the response as JSON.
        let body = response.text().await?;
        let candlesticks: Vec<Vec<serde_json::Value>> = serde_json::from_str(&body)?;

        // Extracting candlesticks closed price.
        //  candlesticks = [
        //      Open time, Open price, High price, Low price, Close price,
        //      Volume, Close time, Quote asset volume, Number of trades,
        //      Taker buy base volume, Taker buy quote volume, Ignore
        //  ];
        let mut record: Vec<DataPoint> = Vec::new();
        for candle in &candlesticks {
            let price = match candle.get(4) {
                Some(data) => data.as_str().and_then(|s| s.parse::<f64>().ok()).unwrap(),
                _ => 0.0,
            };
            let time = match candle.get(6) {
                Some(data) => data.to_string(),
                _ => "None".to_string(),
            };
            record.push(DataPoint::Crypto { price, time });
        }

        Ok(record)
    }

    /// Fetch given trading symbol Price & timestamp
    ///
    /// # Parameters
    /// - symbol: Ticket ID
    /// - interval: time interval per candlestick [1m, 5m, 15m ...]
    /// - limit: Number of candlesticks per fetch.
    ///
    /// # Returns
    /// List of 'DataPoint'
    pub async fn fetch_market_data(symbol: &str, interval: &str, limit: u8) -> Vec<DataPoint> {
        match MarketData::market_data(symbol, interval, limit).await {
            Ok(data) => data,
            Err(_) => Vec::new(),
        }
    }
}
