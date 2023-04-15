mod game;
mod message_broker;
mod server;
mod twitch;
mod websocket;

use actix::{Actor, Addr};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde::Deserialize;
use server::Server;
use twitch::{Channel, TwitchGuy};

use crate::{message_broker::MessageBroker, websocket::WebsocketGuy};

#[derive(Deserialize)]
struct QueryParameter {
    room: String,
}

#[get("/ws")]
async fn index(
    req: HttpRequest,
    stream: web::Payload,
    query: web::Query<QueryParameter>,
) -> Result<HttpResponse, Error> {
    let app_data = req
        .app_data::<AppData>()
        .expect("AppData should always exist i think")
        .clone();

    let channel = query.room.clone();

    app_data
        .twitch_guy_addr
        .do_send(Channel::Join(channel.clone()));
    app_data.server_addr.do_send(Channel::Join(channel.clone()));
    let resp = ws::start(
        WebsocketGuy {
            broker_addr: app_data.broker_addr.clone(),
            channel: channel,
        },
        &req,
        stream,
    );
    resp
}
#[derive(Clone)]
struct AppData {
    broker_addr: Addr<MessageBroker>,
    twitch_guy_addr: Addr<TwitchGuy>,
    server_addr: Addr<Server>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let massage_broker_address = message_broker::MessageBroker::default().start();

    let server_address = Server::new(massage_broker_address.clone()).start();

    let twitch_guy_address = TwitchGuy::new(server_address.clone()).start();
    let app_data = AppData {
        broker_addr: massage_broker_address,
        twitch_guy_addr: twitch_guy_address,
        server_addr: server_address,
    };

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(index))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
