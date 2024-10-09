use std::sync::mpsc::sync_channel;

use actix::prelude::*;
use serde_json::json;

use crate::{OnFillUpdate, OnOrderUpdate, OnOrderbookUpdate, WSMessage, WSStarted};

use super::{binance_ws_connector::BinanceWSConnector, strat::Strat, BinanceWSTag};

pub struct BinanceWSClient {
    subscribe_markets: Option<Vec<String>>,
    strategy_addr: Addr<Strat>,
}

impl BinanceWSClient {
    pub fn blocking_spawn(
        subscribe_markets: Option<Vec<&str>>,
        // api_details: Option<BinanceApiDetails>,
        strategy_addr: Addr<Strat>,
    ) -> Addr<Self> {
        let subscribe_markets =
            subscribe_markets.map(|s| s.iter().map(|s| s.to_string()).collect());

        let (tx, rx) = sync_channel(1);
        Arbiter::new().spawn(async move {
            Self::create(|ctx| {
                BinanceWSConnector::blocking_spawn(
                    BinanceWSTag::OrderbookUpdates,
                    None,
                    true,
                    ctx.address().clone(),
                );

                // if api_details.is_some() {
                //     FtxWSConnector::blocking_spawn(
                //         FtxWSTag::OrderUpdates,
                //         api_details.clone(),
                //         true,
                //         ctx.address().clone(),
                //     );

                //     FtxWSConnector::blocking_spawn(
                //         FtxWSTag::FillUpdates,
                //         api_details.clone(),
                //         true,
                //         ctx.address().clone(),
                //     );
                // } else {
                //     println!("No API details provided, not subscribing order or fill updates");
                // }

                tx.send(ctx.address().clone()).unwrap();
                BinanceWSClient {
                    subscribe_markets,
                    strategy_addr,
                }
            });
        });
        rx.recv().unwrap()
    }
}

impl Actor for BinanceWSClient {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("BinanceWSClient started");
    }
}

impl Handler<WSStarted<BinanceWSTag, BinanceWSConnector>> for BinanceWSClient {
    type Result = ();

    fn handle(
        &mut self,
        msg: WSStarted<BinanceWSTag, BinanceWSConnector>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        println!("BinanceWSClient received WSStarted::{}", msg.tag.as_str());
        match msg.tag {
            BinanceWSTag::OrderbookUpdates => {
                if let Some(markets) = &self.subscribe_markets {
                    for market in markets {
                        msg.ws_address.do_send(WSMessage {
                            message: json!({
                                "method": "SUBSCRIBE",
                                "params": [format!("{}@trade", market)],
                                "id": 1

                            }),
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

impl Handler<WSMessage> for BinanceWSClient {
    type Result = ();

    fn handle(&mut self, msg: WSMessage, _ctx: &mut Self::Context) -> Self::Result {
        println!("{}", msg.message);
        // match msg.message["channel"].as_str() {
        //     Some(channel) => match channel {
        //         "orderbook" => self.strategy_addr.do_send(OnOrderbookUpdate {
        //             message: msg.message,
        //         }),
        //         "orders" => self.strategy_addr.do_send(OnOrderUpdate {
        //             message: msg.message,
        //         }),
        //         "fills" => self.strategy_addr.do_send(OnFillUpdate {
        //             message: msg.message,
        //         }),
        //         _ => panic!("Unknown channel: {}", channel),
        //     },
        //     None => println!("recieved message without channel: {:?}", msg.message),
        // }
    }
}
