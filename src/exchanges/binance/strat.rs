use std::{sync::Arc, time::Duration};

use actix::prelude::*;

use crate::{
    OnFillUpdate, OnOrderUpdate, OnOrderbookUpdate, OnStrategyLoop, OnStrategyStart, Strategy,
};

use super::binance_client::BinanceClient;

pub struct Strat {
    loop_period: Duration,
    binance_client: Arc<BinanceClient>,
    some_state: String,
}

impl Strategy for Strat {
    fn new(ctx: &mut Self::Context) -> Self {
        Strat {
            loop_period: Duration::from_millis(5000),
            binance_client: Arc::new(
                BinanceClient::new()
                    .with_events(Some(vec!["btcusdt", "ethusdt"]), ctx.address().clone()),
            ),
            some_state: "state_beg".to_string(),
        }
    }
}

impl Actor for Strat {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("GridStrategy started");

        ctx.notify(OnStrategyStart {});

        ctx.run_interval(self.loop_period, |_act, ctx| ctx.notify(OnStrategyLoop {}));
    }
}

impl Handler<OnStrategyStart> for Strat {
    type Result = ();

    fn handle(&mut self, _msg: OnStrategyStart, _ctx: &mut Self::Context) -> Self::Result {
        println!("GridStrategy OnStrategyStart");
    }
}

impl Handler<OnStrategyLoop> for Strat {
    type Result = ();

    fn handle(&mut self, _msg: OnStrategyLoop, _ctx: &mut Self::Context) -> Self::Result {
        println!("\nGridStrategy OnStrategyLoop\n");
    }
}

impl Handler<OnOrderbookUpdate> for Strat {
    type Result = ();

    fn handle(&mut self, msg: OnOrderbookUpdate, ctx: &mut Self::Context) -> Self::Result {
        // println!("ob update: {:?}", msg.message);
    }
}

impl Handler<OnOrderUpdate> for Strat {
    type Result = ();

    fn handle(&mut self, msg: OnOrderUpdate, ctx: &mut Self::Context) -> Self::Result {
        println!("order update: {:?}", msg.message);
    }
}

impl Handler<OnFillUpdate> for Strat {
    type Result = ();

    fn handle(&mut self, msg: OnFillUpdate, ctx: &mut Self::Context) -> Self::Result {
        println!("fill update: {:?}", msg.message);
    }
}
