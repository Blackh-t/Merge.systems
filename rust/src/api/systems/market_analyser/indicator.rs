use super::market_data::*;
use core::f64;

/// RSI Calculator based on historical price data for 72 samples.
///
/// # Parameters
/// - symbol: The trading tickets
/// - interval: time interval per candlestick
/// - time_periode: number of candlestick.
///
/// # Returns
/// The lastest calculated RSI.
async fn calculate_rsi(symbol: &str, interval: &str, time_periode: usize) -> f64 {
    // Fetch 72 lastest prices.
    let prices = fetch_prices(symbol, interval, 72).await.unwrap();
    let mut result = 0.0;
    for i in time_periode..(prices.len()) {
        let mut gain = 0.0;
        let mut loss = 0.0;
        let mut curr_price = 0.0;

        // calculate RSI
        for j in (i - time_periode)..i {
            let delta = prices[j + 1] - prices[j];
            if delta > 0.0 {
                gain += delta;
            } else {
                loss -= delta;
            }
            curr_price = prices[j + 1];
        }
        let gain_avg = gain / time_periode as f64;
        let loss_avg = loss / time_periode as f64;
        let rs = gain_avg / loss_avg;
        let rsi = 100.0 - (100.0 / (1.0 + rs));

        // Update the lastest RSI
        result = rsi;
        println!("RSI: {} Price: {}", format!("{:.2}", rsi), curr_price);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rsi() {
        let result = calculate_rsi("ETH", "5m", 14).await;
        assert!(result > -0.1);
    }
}
