use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct FtxPostOrderValue {
    pub market: String,
    pub side: String,
    pub price: f64,
    pub order_type: String,
    pub size: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ioc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_on_price_band: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_after_ts: Option<u64>,
}

struct OnExchangeLimitOrder {
    pub id: String,
    pub state: String,
    pub order_type: String,
    pub market: String,
    pub side: String,
    pub price: f64,
    pub size: f64,
    pub filled_size: f64,
}
