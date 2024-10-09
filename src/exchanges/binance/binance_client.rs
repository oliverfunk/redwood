use actix::Addr;

use super::{binance_ws_client::BinanceWSClient, strat::Strat};

pub struct BinanceClient {
    // rest_connector: BinanceRestConnector,
    ws_client_addr: Option<Addr<BinanceWSClient>>,
}

impl BinanceClient {
    pub fn new() -> Self {
        BinanceClient {
            ws_client_addr: None,
        }
    }

    pub fn with_events(
        mut self,
        subscribe_markets: Option<Vec<&str>>,
        // api_details: Option<FtxApiDetails>,
        strategy_addr: Addr<Strat>,
    ) -> Self {
        let ws_client_addr = BinanceWSClient::blocking_spawn(subscribe_markets, strategy_addr);
        self.ws_client_addr = Some(ws_client_addr);
        self
    }
}
