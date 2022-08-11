use std::time::Duration;

use crate::{OnFillUpdate, OnOrderUpdate, OnOrderbookUpdate, OnStrategyLoop, Strategy};

use super::{FtxApiDetails, FtxClient};
use actix::prelude::*;

pub struct Strat {
    loop_period: Duration,
    ftx_client: FtxClient,
}

impl Strategy for Strat {
    fn new(ctx: &mut Self::Context) -> Self {
        let api_details = Some(FtxApiDetails::from_values(
            "lobwmG4KDR0wTQ4Qs_0VbppYjkCm8OqT7mg8XFW0",
            "c72V6VMHbrU_SyTiTufm_OoviV5Nddeki_0H8TYH",
            "algo_trading",
        ));
        Strat {
            loop_period: Duration::from_millis(200),
            ftx_client: FtxClient::new(api_details.clone()).with_events(
                Some(vec!["BTC-PERP", "BTC/USD"]),
                api_details.clone(),
                ctx.address().clone(),
            ),
        }
    }
}

impl Actor for Strat {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("GridStrategy started");

        ctx.run_interval(self.loop_period, |_act, ctx| ctx.notify(OnStrategyLoop {}));
    }
}

impl Handler<OnStrategyLoop> for Strat {
    type Result = ();

    fn handle(&mut self, _: OnStrategyLoop, ctx: &mut Self::Context) -> Self::Result {
        println!("strat loop");
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
