use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web_actors::ws;
use anyhow::Result;

use crate::message_broker::{Broker, MessageBroker};

/// Define HTTP actor
pub struct WebsocketGuy {
    pub broker_addr: Addr<MessageBroker>,
    pub channel: String,
}

impl Actor for WebsocketGuy {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.broker_addr.do_send(Broker::Subscribe {
            channel: self.channel.clone().into(),
            ws: ctx.address(),
        })
    }
}

pub struct Payload(pub String);

impl Message for Payload {
    type Result = Result<()>;
}

impl Handler<Payload> for WebsocketGuy {
    type Result = Result<()>;

    fn handle(&mut self, msg: Payload, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
        Ok(())
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketGuy {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            _ => (),
        }
    }
}
