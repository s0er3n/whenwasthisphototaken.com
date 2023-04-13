use actix::{Actor, Context};

pub struct Game;

impl Actor for Game {
    type Context = Context<Self>;
}
