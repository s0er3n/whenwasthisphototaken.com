use actix::{Actor, Handler, Message, StreamHandler};
use actix_web_actors::ws;
use anyhow::Result;
use serde::Serialize;

/// Define HTTP actor
pub struct WebsocketGuy;

impl Actor for WebsocketGuy {
    type Context = ws::WebsocketContext<Self>;
}

#[derive(Serialize)]
enum State {
    Test,
}

impl Message for State {
    type Result = Result<()>;
}

impl Handler<State> for WebsocketGuy {
    type Result = Result<()>;

    fn handle(&mut self, msg: State, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg)?);
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
