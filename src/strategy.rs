use actix::prelude::*;

pub trait Strategy: Actor
where
    Self: Actor<Context = Context<Self>>,
{
    fn new(ctx: &mut Self::Context) -> Self;
    fn run() {
        Arbiter::new().spawn(async move {
            Self::create(|ctx| Self::new(ctx));
        });
    }
}
