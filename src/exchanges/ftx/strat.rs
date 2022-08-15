use std::{sync::Arc, time::Duration};

use crate::{
    exchanges::ftx::ftx_value_objects::FtxPostOrderValue, OnFillUpdate, OnOrderUpdate,
    OnOrderbookUpdate, OnStrategyLoop, OnStrategyStart, Strategy,
};

use super::{FtxApiDetails, FtxClient};
use actix::prelude::*;

pub struct Strat {
    loop_period: Duration,
    ftx_client: Arc<FtxClient>,
    some_state: String,
}

impl Strategy for Strat {
    fn new(ctx: &mut Self::Context) -> Self {
        let api_details = Some(FtxApiDetails::from_values(
            "lobwmG4KDR0wTQ4Qs_0VbppYjkCm8OqT7mg8XFW0",
            "c72V6VMHbrU_SyTiTufm_OoviV5Nddeki_0H8TYH",
            "algo_trading",
        ));
        Strat {
            loop_period: Duration::from_millis(5000),
            ftx_client: Arc::new(FtxClient::new(api_details.clone()).with_events(
                Some(vec!["BTC-PERP", "BTC/USD"]),
                api_details.clone(),
                ctx.address().clone(),
            )),
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

    fn handle(&mut self, _: OnStrategyStart, ctx: &mut Self::Context) -> Self::Result {
        println!("strat start");

        let ftx_client = self.ftx_client.clone();
        async move {
            let po = ftx_client
                .post_order(FtxPostOrderValue {
                    market: "BTC-PERP".to_string(),
                    side: "buy".to_string(),
                    price: 23000.0,
                    order_type: "limit".to_string(),
                    size: 0.01,
                    reduce_only: None,
                    ioc: None,
                    post_only: Some(true),
                    client_id: None,
                    reject_on_price_band: None,
                    reject_after_ts: None,
                })
                .await;

            let order_id = po["id"].as_u64().unwrap();
            let co = ftx_client.cancel_order(order_id).await;

            println!("{:?}", co);
        }
        .into_actor(self)
        .map(|res, act, ctx| {
            println!("here");
            act.some_state = "get rekt".to_string();
        })
        .spawn(ctx);
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
