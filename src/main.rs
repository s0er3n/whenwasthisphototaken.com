mod game;
mod message_broker;
mod server;
mod twitch;
mod websocket;

use actix::{Actor, Addr};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use server::Server;
use twitch::{Channel, TwitchGuy};

use crate::{message_broker::MessageBroker, websocket::WebsocketGuy};

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        WebsocketGuy {
            broker_addr: req
                .app_data::<Addr<MessageBroker>>()
                .expect("broker should always exist i think")
                .clone(),
        },
        &req,
        stream,
    );
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = Server::default().start();

    let twitch_guy_address = TwitchGuy::new(server_address).start();

    // joining hardcoded for now
    twitch_guy_address.do_send(Channel::Join("soeren_______".into()));

    let massage_broker_address = message_broker::MessageBroker::default().start();

    HttpServer::new(move || {
        App::new()
            .app_data(massage_broker_address.clone())
            .route("/ws/", web::get().to(index))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
