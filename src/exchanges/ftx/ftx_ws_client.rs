use crate::{messages::*, Strategy};
use actix::prelude::*;
use serde_json::{json, Value};
use std::sync::mpsc::sync_channel;

use super::{ftx_ws_connector::FtxWSConnector, strat::Strat, FtxApiDetails, FtxWSTag};

pub struct FtxWSClient {
    subscribe_markets: Option<Vec<String>>,
    strategy_addr: Addr<Strat>,
}

impl FtxWSClient {
    pub fn blocking_spawn(
        subscribe_markets: Option<Vec<&str>>,
        api_details: Option<FtxApiDetails>,
        strategy_addr: Addr<Strat>,
    ) -> Addr<Self> {
        let subscribe_markets =
            subscribe_markets.map(|s| s.iter().map(|s| s.to_string()).collect());

        let (tx, rx) = sync_channel(1);
        Arbiter::new().spawn(async move {
            Self::create(|ctx| {
                FtxWSConnector::blocking_spawn(
                    FtxWSTag::OrderbookUpdates,
                    None,
                    true,
                    ctx.address().clone(),
                );

                if api_details.is_some() {
                    FtxWSConnector::blocking_spawn(
                        FtxWSTag::OrderUpdates,
                        api_details.clone(),
                        true,
                        ctx.address().clone(),
                    );

                    FtxWSConnector::blocking_spawn(
                        FtxWSTag::FillUpdates,
                        api_details.clone(),
                        true,
                        ctx.address().clone(),
                    );
                } else {
                    println!("No API details provided, not subscribing order or fill updates");
                }

                tx.send(ctx.address().clone()).unwrap();
                FtxWSClient {
                    subscribe_markets,
                    strategy_addr,
                }
            });
        });
        rx.recv().unwrap()
    }
}

impl Actor for FtxWSClient {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("FtxWSClient started");
    }
}

impl Handler<WSStarted<FtxWSTag, FtxWSConnector>> for FtxWSClient {
    type Result = ();

    fn handle(
        &mut self,
        msg: WSStarted<FtxWSTag, FtxWSConnector>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        match msg.tag {
            FtxWSTag::OrderbookUpdates => {
                println!("Starting orderbook updates ws");
                if let Some(subscribe_markets) = self.subscribe_markets.as_ref() {
                    for market in subscribe_markets {
                        msg.ws_address.do_send(WSMessage {
                            message: json!({
                                "op": "subscribe",
                                "channel": "orderbook",
                                "market": market,
                            }),
                        });
                    }
                }
            }
            FtxWSTag::OrderUpdates => {
                println!("Starting order updates ws");
                msg.ws_address.do_send(WSMessage {
                    message: json!({
                        "op": "subscribe",
                        "channel": "orders",
                    }),
                });
            }
            FtxWSTag::FillUpdates => {
                println!("Starting fill updates ws");
                msg.ws_address.do_send(WSMessage {
                    message: json!({
                        "op": "subscribe",
                        "channel": "fills",
                    }),
                });
            }
        }
    }
}

impl Handler<WSMessage> for FtxWSClient {
    type Result = ();

    fn handle(&mut self, msg: WSMessage, _ctx: &mut Self::Context) -> Self::Result {
        match msg.message["channel"].as_str() {
            Some(channel) => match channel {
                "orderbook" => self.strategy_addr.do_send(OnOrderbookUpdate {
                    message: msg.message,
                }),
                "orders" => self.strategy_addr.do_send(OnOrderUpdate {
                    message: msg.message,
                }),
                "fills" => self.strategy_addr.do_send(OnFillUpdate {
                    message: msg.message,
                }),
                _ => panic!("Unknown channel: {}", channel),
            },
            None => println!("recieved message without channel: {:?}", msg.message),
        }
    }
}
