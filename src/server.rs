use std::collections::HashMap;

use actix::{Actor, Context, Handler};

use crate::{game::Game, twitch::TwitchMessage};

pub struct Server {
    games: HashMap<String, Game>,
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
        dbg!(msg.0);
    }
}
