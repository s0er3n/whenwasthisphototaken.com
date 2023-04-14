use std::collections::HashMap;

use actix::{Actor, Context, Handler, Message};

use twitch_irc::message::PrivmsgMessage;

enum GameState {
    Image,
    AfterImage,
    Results,
}
struct Image {
    url: String,
    result: u16,
    description: String,
    // string -> sender
    guesses: HashMap<String, u16>,
    scores: HashMap<String, f64>,
}

pub struct Game {
    state: GameState,
    images: Vec<Image>,
    round_number: u8,
}

impl Actor for Game {
    type Context = Context<Self>;
}

pub struct TwitchMsg {
    msg: String,
    author: String,
    author_id: String,
}

impl Message for TwitchMsg {
    type Result = ();
}

impl From<PrivmsgMessage> for TwitchMsg {
    fn from(msg: PrivmsgMessage) -> Self {
        Self {
            msg: msg.message_text,
            author: msg.sender.name,
            author_id: msg.sender.id,
        }
    }
}

impl Handler<TwitchMsg> for Game {
    type Result = ();

    fn handle(&mut self, msg: TwitchMsg, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
