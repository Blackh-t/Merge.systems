use core::f64;
use reqwest;
use std::error::Error;

/// Sending a HTTP request to binance then extracting the closed prices into a list.
///
/// # Parameters
/// - symbol: Ticket ID
/// - interval: time interval per candlestick [1m, 5m, 15m ...]
/// - limit: Number of candlesticks per fetch.
///
/// # Return
/// - A list of candlesticks closed price.
pub async fn fetch_prices(
    symbol: &str,
    interval: &str,
    limit: u8,
) -> Result<Vec<f64>, Box<dyn Error>> {
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
    let mut closed_prices = Vec::new();
    for candle in &candlesticks {
        if let Some(price) = candle.get(4) {
            let price = price.as_str().and_then(|s| s.parse::<f64>().ok());
            closed_prices.push(price.unwrap());
        }
    }

    Ok(closed_prices)
}
