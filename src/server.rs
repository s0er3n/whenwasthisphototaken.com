use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};

use crate::{
    game::{Game, TwitchMsg},
    message_broker::MessageBroker,
    twitch::TwitchMessage,
};

pub struct Server {
    games: HashMap<String, Addr<Game>>,
    broker_addr: Addr<MessageBroker>,
}

impl Server {
    pub fn new(broker_addr: Addr<MessageBroker>) -> Self {
        Server {
            games: HashMap::new(),
            broker_addr,
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<TwitchMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: TwitchMessage, ctx: &mut Self::Context) -> Self::Result {
        if let twitch_irc::message::ServerMessage::Privmsg(msg) = msg.0 {
            if let Some(game) = self.games.get(&msg.channel_login) {
                game.do_send(TwitchMsg::from(msg));
            } else {
                let game = Game::new(self.broker_addr.clone(), msg.channel_login.clone()).start();
                self.games.insert(msg.channel_login.clone(), game.clone());
                game.do_send(TwitchMsg::from(msg));
            }
        }
    }
}
