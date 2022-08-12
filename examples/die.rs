use std::{sync::mpsc::sync_channel, time::Duration};

use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
struct Die;

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify_later(Die {}, Duration::from_secs(3));
    }
}

// To use actor with supervisor actor has to implement `Supervised` trait
impl actix::Supervised for MyActor {
    fn restarting(&mut self, ctx: &mut Self::Context) {
        println!("restarting");
    }
}

impl Handler<Die> for MyActor {
    type Result = ();

    fn handle(&mut self, _: Die, ctx: &mut Context<MyActor>) {
        ctx.stop();
    }
}

fn main() {
    let mut sys = System::new();

    let addr = sys.block_on(async {
        let (tx, rx) = sync_channel(1);
        let arbiter = Arbiter::new();
        let arbiter_handle = arbiter.handle().clone();
        Supervisor::start_in_arbiter(&arbiter_handle, move |ctx| {
            tx.send(ctx.address().clone()).unwrap();
            MyActor {}
        });
        rx.recv().unwrap()
    });
    // addr.do_send(Die);

    sys.run();
}
