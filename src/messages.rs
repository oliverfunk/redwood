use actix::prelude::*;
use serde_json::Value;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WSStarted<T, A: Actor> {
    pub tag: T,
    pub ws_address: Addr<A>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WSMessage {
    pub message: Value,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct OnStrategyLoop {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct OnOrderbookUpdate {
    pub message: Value,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct OnOrderUpdate {
    pub message: Value,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct OnFillUpdate {
    pub message: Value,
}
