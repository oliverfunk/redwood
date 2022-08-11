use actix::prelude::*;
use quantm_trade::{exchanges::ftx::Strat, Strategy};

// struct GridStrategy {
//     ftx_client: i32,
// }

// impl Actor for GridStrategy {
//     type Context = Context<Self>;

//     fn started(&mut self, ctx: &mut Self::Context) {
//         println!("GridStrategy started");
//     }
// }

// impl Strategy for GridStrategy {
//     fn new(ctx: &mut Self::Context) -> Self {
//         GridStrategy { ftx_client: 1 }
//     }
// }

fn main() {
    let sys = System::new();
    sys.block_on(async move {
        Strat::run();
    });
    sys.run().unwrap();
}
