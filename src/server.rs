use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};

use crate::{game::Game, twitch::TwitchMessage};

pub struct Server {
    games: HashMap<String, Addr<Game>>,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            games: HashMap::new(),
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
                todo!("send sender and msg to game for it to handle");
                // game.do_send();
            }
        }
    }
}
