mod ftx_client;
mod ftx_rest_connector;
mod ftx_ws_client;
mod ftx_ws_connector;
mod strat;

// public exports
pub use ftx_client::FtxClient;
pub use strat::Strat;

use hmac::Hmac;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct FtxApiDetails {
    api_key: String,
    api_secret: String,
    api_subaccount: String,
}

impl FtxApiDetails {
    pub fn from_values(api_key: &str, api_secret: &str, api_subaccount: &str) -> Self {
        FtxApiDetails {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            api_subaccount: api_subaccount.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FtxWSTag {
    OrderbookUpdates,
    OrderUpdates,
    FillUpdates,
}

impl FtxWSTag {
    fn as_str(&self) -> &str {
        match self {
            FtxWSTag::OrderbookUpdates => "orderbook_update",
            FtxWSTag::OrderUpdates => "order_update",
            FtxWSTag::FillUpdates => "fill_update",
        }
    }

    fn from_str(tag: &str) -> Self {
        match tag {
            "orderbook_update" => FtxWSTag::OrderbookUpdates,
            "order_update" => FtxWSTag::OrderUpdates,
            "fill_update" => FtxWSTag::FillUpdates,
            _ => panic!("Unknown tag: {}", tag),
        }
    }
}
