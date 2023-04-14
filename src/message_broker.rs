use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};

use crate::websocket::WebsocketGuy;

pub struct MessageBroker {
    subscribers: HashMap<String, Vec<Addr<WebsocketGuy>>>,
}

impl Handler<Broker> for MessageBroker {
    type Result = ();

    fn handle(&mut self, msg: Broker, ctx: &mut Self::Context) -> Self::Result {
        // will always match for now but thats ok :)
        if let Broker::Subscribe { ws, channel } = msg {
            self.subcribe(channel, ws);
        }
    }
}

impl Default for MessageBroker {
    fn default() -> Self {
        MessageBroker {
            subscribers: HashMap::new(),
        }
    }
}

impl MessageBroker {
    fn subcribe(&mut self, channel: String, addr: Addr<WebsocketGuy>) {
        self.subscribers
            .entry(channel)
            .or_insert_with(Vec::new)
            .push(addr);
    }
}

impl Actor for MessageBroker {
    type Context = Context<Self>;
}

pub enum Broker {
    Subscribe {
        ws: Addr<WebsocketGuy>,
        channel: String,
    },
}

impl Message for Broker {
    type Result = ();
}
