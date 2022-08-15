use super::FtxApiDetails;

use super::ftx_rest_connector::FtxRestConnector;
use super::ftx_value_objects::FtxPostOrderValue;
use super::{ftx_ws_client::FtxWSClient, strat::Strat};
use actix::Addr;
use serde_json::{json, Value};

pub struct FtxClient {
    rest_connector: FtxRestConnector,
    ws_client_addr: Option<Addr<FtxWSClient>>,
}

impl FtxClient {
    pub fn new(api_details: Option<FtxApiDetails>) -> Self {
        FtxClient {
            rest_connector: FtxRestConnector::new(api_details),
            ws_client_addr: None,
        }
    }

    pub fn with_events(
        mut self,
        subscribe_markets: Option<Vec<&str>>,
        api_details: Option<FtxApiDetails>,
        strategy_addr: Addr<Strat>,
    ) -> Self {
        let ws_client_addr =
            FtxWSClient::blocking_spawn(subscribe_markets, api_details, strategy_addr);
        self.ws_client_addr = Some(ws_client_addr);
        self
    }

    pub async fn get_orders(&self, market: &str) -> Value {
        self.rest_connector
            .get("orders", Some(&mut [("market", market)]))
            .await
            .expect("failed to get orders for market: {market}")
    }

    pub async fn get_markets(&self) -> Value {
        self.rest_connector
            .get("markets", None)
            .await
            .expect("failed to get markets")
    }

    pub async fn post_order(&self, values: FtxPostOrderValue) -> Value {
        self.rest_connector
            .post("orders", None, Some(json!(values)))
            .await
            .expect("failed to post order")
    }

    pub async fn cancel_order(&self, order_id: u64) -> Value {
        self.rest_connector
            .delete(format!("orders/{order_id}").as_str(), None)
            .await
            .expect("failed to cancel order")
    }
}
