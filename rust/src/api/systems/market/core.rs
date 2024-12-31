pub use super::error::MarketResults;
use serde::{Deserialize, Serialize};
use serde_json;

/// DataPoint represents a single even in the market timeline.
#[derive(Debug, Serialize, Deserialize)]
pub enum DataPoint {
    Crypto {
        price: f64,
        time: String,
    },
    RSI {
        value: f64,
        time: String,
        price: f64,
    },
}

/// MarketData holds all recorded market events
#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub record: Vec<DataPoint>,
}

impl MarketData {
    /// Create an new empty 'MaarketData' instance
    pub fn new() -> Self {
        MarketData { record: Vec::new() }
    }

    /// Serialize 'MarketData' into JSON String form.
    pub fn serialize(&self) -> MarketResults<String> {
        let data = serde_json::to_string_pretty(self)?;
        Ok(data)
    }
}
