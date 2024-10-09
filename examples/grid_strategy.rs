use actix::prelude::*;
use redwood::{exchanges::binance::Strat, Strategy};

fn main() {
    let sys = System::new();
    sys.block_on(async move {
        Strat::run();
    });
    sys.run().unwrap();
}
