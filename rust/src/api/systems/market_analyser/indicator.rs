use super::{core::MarketData, market_data::*};
use crate::api::systems::market_analyser::core::DataPoint;
use chrono::{DateTime, Utc};
use core::{f64, panic};

impl MarketData {
    /// RSI Calculator based on historical price data for 72 samples.
    ///
    /// # Parameters
    /// - symbol: The trading tickets
    /// - interval: time interval per candlestick
    /// - time_periode: number of candlestick.
    ///
    /// # Returns
    /// The lastest calculated RSI.
    async fn get_rsi(symbol: &str, interval: &str, time_periode: usize) -> Self {
        // Fetch 72 lastest prices.
        let prices_x = match MarketData::fetch_prices(symbol, interval, 72).await {
            Ok(data) => data,
            Err(err) => panic!("Price Fetch: {}", err),
        };

        let mut collector = MarketData::new();

        // Calculation
        // - First loop: Defined Base Pointer.
        // - Secound loop: accmulate market price to find RS.
        for i in time_periode..(prices_x.len()) {
            let mut gain = 0.0;
            let mut loss = 0.0;
            let mut current_price = 0.0;
            let mut current_time = String::new();

            // Collect Gain & Loss data
            for j in (i - time_periode)..i {
                // Unpacking data out of Enum
                let prev_price = match &prices_x[j] {
                    DataPoint::Crypto { price, .. } => *price,
                    _ => 0.0,
                };
                let (c_price, c_time) = match &prices_x[j + 1] {
                    DataPoint::Crypto { price, time } => (*price, time.clone()),
                    _ => (0.0, String::new()),
                };

                // Calculate and accmulate the Δ.
                let delta = c_price - prev_price;
                if delta > 0.0 {
                    gain += delta;
                } else {
                    loss -= delta;
                }

                // Update the current price and time for future analyzer
                current_price = c_price;
                current_time = c_time;
            }

            // Calculate RSI: Find RS througt the average value of delta.
            let gain_avg = gain / time_periode as f64;
            let loss_avg = loss / time_periode as f64;
            let rs = gain_avg / loss_avg;
            let rsi = 100.0 - (100.0 / (1.0 + rs));

            // Time Converting
            let timestam = current_time.parse().unwrap_or(0);
            let naive_time = DateTime::from_timestamp(timestam / 1000, 0).unwrap();
            let time = naive_time.format("%Y-%m-%d %H:%M").to_string();

            // Store the value.
            collector.record.push(DataPoint::RSI {
                value: rsi,
                time,
                price: current_price,
            });
        }
        collector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rsi() {
        let result = MarketData::get_rsi("ETH", "1h", 14).await;
        let collector = result.serialize().unwrap();
        println!("{}", collector);
    }
}
