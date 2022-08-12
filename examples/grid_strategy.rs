use actix::prelude::*;
use quantm_trade::{exchanges::ftx::Strat, Strategy};

fn main() {
    let sys = System::new();
    sys.block_on(async move {
        Strat::run();
    });
    sys.run().unwrap();
}
