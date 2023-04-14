use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};

use crate::websocket::{Payload, WebsocketGuy};

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

    fn distribute_message(&self, msg: BrokerMessage) {
        if let Some(subcribers) = self.subscribers.get(&msg.channel) {
            subcribers.iter().for_each(|subscriber| {
                subscriber.do_send(Payload(msg.payload.clone()));
            })
        }
    }
}

struct BrokerMessage {
    channel: String,
    payload: String,
}

impl Message for BrokerMessage {
    type Result = ();
}

impl Handler<BrokerMessage> for MessageBroker {
    type Result = ();
    fn handle(&mut self, msg: BrokerMessage, _: &mut Self::Context) -> Self::Result {
        self.distribute_message(msg);
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