mod auth;
mod game;
mod message_broker;
mod server;
mod twitch;
mod websocket;

use actix::{Actor, Addr};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use anyhow::Result;
use auth::Auth;
use serde::Deserialize;
use server::Server;
use twitch::{Channel, TwitchGuy};

use crate::{message_broker::MessageBroker, websocket::WebsocketGuy};

#[derive(Deserialize)]
struct QueryParameter {
    code: String,
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

    let code = query.code.clone();

    let channel: String = match app_data.auth.check_code(code).await {
        Ok(channel) => channel,
        Err(err) => {
            return Err(actix_web::error::ErrorInternalServerError(err.to_string()));
        }
    };
    dbg!(&channel);

    app_data
        .twitch_guy_addr
        .do_send(Channel::Join(channel.clone()));
    app_data.server_addr.do_send(Channel::Join(channel.clone()));
    let resp = ws::start(
        WebsocketGuy {
            broker_addr: app_data.broker_addr.clone(),
            channel,
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
    auth: Auth,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let broker_addr = message_broker::MessageBroker::default().start();

    let server_addr = Server::new(broker_addr.clone()).start();

    let auth = Auth::new();

    let twitch_guy_addr = TwitchGuy::new(server_addr.clone()).start();
    let app_data = AppData {
        broker_addr,
        twitch_guy_addr,
        server_addr,
        auth,
    };

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(index))
        .bind(("0.0.0.0", 3030))?
        .run()
        .await
}
