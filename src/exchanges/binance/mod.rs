mod binance_client;
mod binance_rest_connector;
mod binance_ws_client;
mod binance_ws_connector;
mod strat;

pub use strat::Strat;

#[derive(Debug, Clone)]
pub struct BinanceApiDetails {
    api_key: String,
    api_secret: String,
    api_subaccount: String,
}

impl BinanceApiDetails {
    pub fn from_values(api_key: &str, api_secret: &str, api_subaccount: &str) -> Self {
        BinanceApiDetails {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            api_subaccount: api_subaccount.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinanceWSTag {
    OrderbookUpdates,
    OrderUpdates,
    FillUpdates,
}

impl BinanceWSTag {
    fn as_str(&self) -> &str {
        match self {
            BinanceWSTag::OrderbookUpdates => "orderbook_update",
            BinanceWSTag::OrderUpdates => "order_update",
            BinanceWSTag::FillUpdates => "fill_update",
        }
    }
}
