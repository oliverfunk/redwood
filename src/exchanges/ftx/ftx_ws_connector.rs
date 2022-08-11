use std::sync::mpsc::sync_channel;

use actix::io::{SinkWrite, WriteHandler};
use actix::prelude::*;
use futures::stream::SplitSink;
use futures::stream::SplitStream;
use futures::StreamExt;
use hex::ToHex;
use hmac::Mac;
use serde_json::json;
use std::time::{Duration, SystemTime};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::{
    connect_async_tls_with_config, tungstenite::Message, Connector, MaybeTlsStream, WebSocketStream,
};

use crate::messages::{WSMessage, WSStarted};

use super::{ftx_ws_client::FtxWSClient, FtxApiDetails};
use super::{FtxWSTag, HmacSha256};

type WSSend = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

const FTX_WS_ENDPOINT: &str = "wss://ftx.com/ws/";
const FTX_WS_PING_PERIOD_SECS: u8 = 15;
pub struct FtxWSConnector {
    tag: FtxWSTag,

    api_details: Option<FtxApiDetails>,
    ping_enabled: bool,

    client_addr: Addr<FtxWSClient>,
    ws_send: Option<SinkWrite<Message, WSSend>>,
}

impl FtxWSConnector {
    pub fn blocking_spawn(
        tag: FtxWSTag,
        api_details: Option<FtxApiDetails>,
        ping_enabled: bool,
        client_addr: Addr<FtxWSClient>,
    ) -> Addr<Self> {
        let (tx, rx) = sync_channel(1);
        Arbiter::new().spawn(async move {
            FtxWSConnector::create(|ctx| {
                tx.send(ctx.address().clone()).unwrap();
                FtxWSConnector {
                    tag,
                    api_details,
                    ping_enabled,
                    client_addr,
                    ws_send: None,
                }
            });
        });
        rx.recv().unwrap()
    }

    async fn create_websocket() -> (
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) {
        let ws_endpoint = url::Url::parse(FTX_WS_ENDPOINT).unwrap();

        let (ws_connection, _) = connect_async_tls_with_config(
            ws_endpoint,
            None,
            Some(Connector::NativeTls(
                native_tls::TlsConnector::new().unwrap(),
            )),
        )
        .await
        .expect("Failed to connect to FTX WebSocket endpoint");

        ws_connection.split()
    }

    fn ping_message() -> Message {
        Message::Text(json!({"op": "ping"}).to_string())
    }

    fn auth_message(&self) -> Message {
        if let Some(api_details) = &self.api_details {
            let now_in_millis = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();

            let mut mac = HmacSha256::new_from_slice(api_details.api_secret.as_bytes())
                .expect("HMAC can take key of any size");
            mac.update(format!("{}websocket_login", now_in_millis).as_bytes());
            let signature = mac.finalize().into_bytes().encode_hex::<String>();

            Message::Text(
                json!({
                    "op": "login",
                    "args": {
                        "subaccount": api_details.api_subaccount,
                        "key": api_details.api_key,
                        "sign": signature,
                        "time": now_in_millis as u64,
                    }
                })
                .to_string(),
            )
        } else {
            panic!("Cannot auth websocket, no API details provided");
        }
    }

    fn start_message(&self, ctx: &<Self as Actor>::Context) -> WSStarted<FtxWSTag, Self> {
        WSStarted {
            tag: self.tag.clone(),
            ws_address: ctx.address().clone(),
        }
    }
}

impl Actor for FtxWSConnector {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        FtxWSConnector::create_websocket()
            .into_actor(self)
            .map(|result, actor, ctx| {
                // set connections
                let (ws_send, ws_receive) = result;
                ctx.add_stream(ws_receive);
                actor.ws_send = Some(SinkWrite::new(ws_send, ctx));

                // auth websocket
                if actor.api_details.is_some() {
                    let auth_msg = actor.auth_message();
                    match actor.ws_send.as_mut().unwrap().write(auth_msg) {
                        Ok(_) => println!("Websocket auth'd"),
                        Err(e) => {
                            println!("FtxWSConnector could not auth websocket: {}", e);
                            ctx.stop();
                        }
                    }
                }
            })
            .then(|_result, actor, ctx| {
                // tell the client we're ready
                let (client_addr, start_message) =
                    (actor.client_addr.clone(), actor.start_message(ctx));
                client_addr.send(start_message).into_actor(actor)
            })
            .map(|result, actor, ctx| match result {
                Ok(_) => println!("FtxWSConnector {} started", actor.tag.as_str()),
                Err(e) => {
                    println!("FtxWSConnector error: {}", e);
                    ctx.stop();
                }
            })
            .wait(ctx);

        if self.ping_enabled {
            ctx.run_interval(
                Duration::from_secs(FTX_WS_PING_PERIOD_SECS.into()),
                |act, ctx| match act
                    .ws_send
                    .as_mut()
                    .unwrap()
                    .write(FtxWSConnector::ping_message())
                {
                    Ok(_) => println!("FtxWSConnector ping sent"),
                    Err(e) => {
                        println!("FtxWSConnector error: {}", e);
                        ctx.stop();
                    }
                },
            );
        }
    }
}

impl Supervised for FtxWSConnector {
    fn restarting(&mut self, _ctx: &mut <Self as Actor>::Context) {
        println!("FtxWSConnector restarting");

        // todo!("need to recreate websocket connection");
    }
}

impl Handler<WSMessage> for FtxWSConnector {
    type Result = ();

    fn handle(&mut self, msg: WSMessage, ctx: &mut Self::Context) -> Self::Result {
        match self
            .ws_send
            .as_mut()
            .unwrap()
            .write(Message::Text(msg.message.to_string()))
        {
            Ok(_) => (),
            Err(e) => {
                println!("Error writing to websocket: {}", e);
                ctx.stop();
            }
        }
    }
}

impl StreamHandler<Result<Message, Error>> for FtxWSConnector {
    fn handle(&mut self, item: Result<Message, Error>, ctx: &mut Self::Context) {
        match item {
            Ok(msg) => match msg.into_text() {
                // todo: do proper deserilaisations
                Ok(msg_text) => match serde_json::from_str(&msg_text) {
                    Ok(json_msg) => self.client_addr.do_send(WSMessage { message: json_msg }),
                    Err(e) => {
                        println!("failed to parse WS message to json: {}", e);
                        ctx.stop();
                    }
                },
                Err(e) => {
                    println!("got non-text value from WS: {}", e);
                    ctx.stop();
                }
            },
            Err(e) => {
                println!("FtxWSConnector error: {}", e);
                ctx.stop();
            }
        }
    }
}

impl WriteHandler<Error> for FtxWSConnector {}
