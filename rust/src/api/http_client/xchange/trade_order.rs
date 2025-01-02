pub use super::error::TradeOrderResult;
use crate::api::systems::market::core::MarketData;
use core::f64;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Futures Trade Order Book.
///
/// # Elements
/// - clientOid: Unique identifier created by the user.
/// - side: buy/sell.
/// - symbol: Trade ticket.
/// - type: Market / Limit.
/// - isIsolated: true-isolated margin ,false-cross margin. defult as false.
/// - size: Number of contract in USDT.
/// - closeOrder: Mark to close positions.
/// - triggerStopUpPrice: TP.
/// - triggerStopDownPrice: SL.
///
/// # NOTE
/// Used serde_json to convert into JSON.
/// This Struct are used as payload into HTTP request body.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeOrder {
    #[serde(rename = "clientOid")]
    pub client_oid: String,
    pub side: String,
    pub symbol: String,
    pub leverage: Option<String>,
    #[serde(rename = "type")]
    pub order_type: Option<String>,
    #[serde(rename = "isIsolated")]
    pub is_isolated: Option<bool>,
    pub size: Option<f64>,
    #[serde(rename = "closeOrder")]
    pub close_order: Option<bool>,
    #[serde(rename = "triggerStopUpPrice")]
    pub trigger_stop_up_price: Option<String>,
    #[serde(rename = "triggerStopDownPrice")]
    pub trigger_stop_down_price: Option<String>,
}

impl TradeOrder {
    /// Init a new TradeOrder with generated clientOid.
    fn new() -> Self {
        TradeOrder {
            client_oid: Uuid::new_v4().to_string(),
            side: String::new(),
            symbol: String::new(),
            leverage: None,
            order_type: None,
            is_isolated: None,
            size: None,
            close_order: None,
            trigger_stop_up_price: None,
            trigger_stop_down_price: None,
        }
    }

    fn long(mut self) -> Self {
        self.side = "buy".to_string();
        self
    }

    fn short(mut self) -> Self {
        self.side = "sell".to_string();
        self
    }

    fn symbol(mut self, ticket: String) -> Self {
        self.symbol = ticket;
        self
    }

    fn market_order(mut self) -> Self {
        self.order_type = Some("market".to_string());
        self
    }

    fn limit_order(mut self) -> Self {
        self.order_type = Some("limit".to_string());
        self
    }

    fn enable_isolated(mut self) -> Self {
        self.is_isolated = Some(true);
        self
    }

    fn set_amount(mut self, amount: f64) -> Self {
        self.size = Some(amount);
        self
    }

    fn close(mut self) -> Self {
        self.close_order = Some(true);
        self
    }

    fn set_sl(mut self, price: String) -> Self {
        self.trigger_stop_down_price = Some(price);
        self
    }

    fn set_tp(mut self, price: String) -> Self {
        self.trigger_stop_up_price = Some(price);
        self
    }

    fn set_leverage(mut self, leverage: String) -> Self {
        self.leverage = Some(leverage);
        self
    }

    /// Placing an Long position with TP&SL are given in percent.
    ///
    /// # Parameters
    /// - symbol: Trading ticket.
    /// - amount: USDT to trade.
    /// - sl_percent: Percentage of loss.
    /// - tp_percentage: Percentage of profit.
    /// - leverage: Factor for the trade.
    ///
    /// # Returns
    /// - Returning a completed configurated payload that can be used in HTTP-request after
    /// serialized.
    pub async fn open_long<T: Into<f64> + Copy, S: Into<String> + Copy>(
        symbol: S,
        amount: T,
        sl_percent: T,
        tp_percent: T,
        leverage: S,
    ) -> Self {
        // Converting String into f64.
        let leverage_parsed = leverage.into().parse::<f64>().unwrap();

        TradeOrder::new()
            .long()
            .set_tp(format!(
                "{}",
                // Sets take profit price based on the market price and the tp_percentage.
                // TP-price = market price * ( 1 + ( percent above the market price / leverage))
                MarketData::get_market_price(symbol.into().as_str()).await
                    * (1.0 + (tp_percent.into() / leverage_parsed))
            ))
            .set_sl(format!(
                "{}",
                MarketData::get_market_price(symbol.into().as_str()).await
                    * (1.0 - (sl_percent.into() / leverage_parsed))
            ))
            .symbol(symbol.into())
            .market_order()
            .set_amount(amount.into())
            .set_leverage(leverage.into())
    }

    /// Placing an Short position with TP&SL are given in percent.
    ///
    /// # Parameters
    /// - symbol: Trading ticket.
    /// - amount: USDT to trade.
    /// - sl_percent: Percentage of loss.
    /// - tp_percentage: Percentage of profit.
    /// - leverage: Factor for the trade.
    ///
    /// # Returns
    /// - Returning a completed configurated payload that can be used in HTTP-request after
    /// serialized.
    pub async fn open_short<T: Into<f64> + Copy, S: Into<String> + Copy>(
        symbol: S,
        amount: T,
        sl_percent: T,
        tp_percent: T,
        leverage: S,
    ) -> Self {
        // Converting String into f64.
        let leverage_parsed = leverage.into().parse::<f64>().unwrap();

        TradeOrder::new()
            .short()
            .set_tp(format!(
                "{}",
                // Sets take profit price based on the market price and the tp_percentage.
                // TP-price = market price * ( 1 - ( percent above the market price / leverage))
                MarketData::get_market_price(symbol.into().as_str()).await
                    * (1.0 - (tp_percent.into() / leverage_parsed))
            ))
            .set_sl(format!(
                "{}",
                MarketData::get_market_price(symbol.into().as_str()).await
                    * (1.0 + (sl_percent.into() / leverage_parsed))
            ))
            .symbol(symbol.into())
            .market_order()
            .set_amount(amount.into())
    }

    /// Closing all open positions base on the given trade symbol.
    pub fn close_orders<T: Into<f64>, S: Into<String>>(symbol: S, amount: T) -> Self {
        TradeOrder::new()
            .short()
            .close()
            .symbol(symbol.into())
            .market_order()
            .set_amount(amount.into())
    }

    /// Converting TradeOrder Struct into JSON-String
    pub fn serialize(&self) -> TradeOrderResult<String> {
        let parsed = serde_json::to_string(&self)?;
        Ok(parsed)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_long_position_payload_generate() {
        let payload = TradeOrder::open_long("BTC", 100, 5, 5, "50")
            .await
            .serialize();
        println!("{}", payload.unwrap());
    }
}
