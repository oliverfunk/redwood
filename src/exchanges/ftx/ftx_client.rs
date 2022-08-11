use super::FtxApiDetails;

use super::ftx_rest_connector::FtxRestConnector;
use super::{ftx_ws_client::FtxWSClient, strat::Strat};
use actix::Addr;
use serde_json::Value;

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

    pub fn get_markets(&self) -> Result<Value, reqwest::Error> {
        todo!()
        // self.rest_connector.get_markets()
    }
}
