use std::u16;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct TradeOrder {
    clientOid: String,
    side: String,
    symbol: String,
    #[serde(rename = "type")]
    order_type: Option<String>,
    isIsolated: Option<bool>,
    funds: Option<String>,
}

impl TradeOrder {
    /// Init TradeOrder with generated clientOid.
    fn new() -> Self {
        TradeOrder {
            clientOid: Uuid::new_v4().to_string(),
            side: String::new(),
            symbol: String::new(),
            order_type: None,
            isIsolated: Some(true),
            funds: None,
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
        self.isIsolated = Some(true);
        self
    }

    fn set_amount(mut self, amount: u16) -> Self {
        self.funds = Some(amount.to_string());
        self
    }
}
