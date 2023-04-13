use std::collections::HashMap;

use actix::{Actor, Context, Handler};

use crate::{game::Game, twitch::TwitchMessage};

pub struct Server {
    games: HashMap<String, Game>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<TwitchMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: TwitchMessage, ctx: &mut Self::Context) -> Self::Result {
        todo!();
    }
}
