use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};

use crate::websocket::{Payload, WebsocketGuy};

pub struct MessageBroker {
    subscribers: HashMap<String, Vec<Addr<WebsocketGuy>>>,
    last_message: HashMap<String, String>,
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
            last_message: HashMap::new(),
        }
    }
}

impl MessageBroker {
    fn subcribe(&mut self, channel: String, addr: Addr<WebsocketGuy>) {
        if self.last_message.contains_key(&channel) {
            addr.do_send(Payload(
                self.last_message
                    .get(&channel)
                    .expect("it should exist if the key exists")
                    .clone(),
            ));
        }
        self.subscribers
            .entry(channel)
            .or_insert_with(Vec::new)
            .push(addr);
    }

    fn distribute_message(&mut self, msg: BrokerMessage) {
        dbg!(&msg);
        self.last_message
            .insert(msg.channel.clone(), msg.payload.clone());
        if let Some(subcribers) = self.subscribers.get(&msg.channel) {
            subcribers.iter().for_each(|subscriber| {
                subscriber.do_send(Payload(msg.payload.clone()));
            })
        }
    }
}

#[derive(Debug)]
pub struct BrokerMessage {
    pub channel: String,
    pub payload: String,
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
