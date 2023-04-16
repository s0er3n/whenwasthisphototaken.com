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

use actix_session::storage::CookieSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::Key;
#[derive(Deserialize)]
struct QueryParameter {
    code: Option<String>,
}

#[get("/ws")]
async fn index(
    req: HttpRequest,
    stream: web::Payload,
    query: web::Query<QueryParameter>,
    session: Session,
) -> Result<HttpResponse, Error> {

    let app_data = req
        .app_data::<AppData>()
        .expect("AppData should always exist i think")
        .clone();

    let code = query.code.clone();


    let channel: String = 
    // FIXME: check for cookie age or maybe if user changed channel name
    if let Ok(Some(channel)) = session.get::<String>("twitch_channel"){
        channel
    } else {
        if code == None {
            return Ok(actix_web::HttpResponse::TemporaryRedirect().insert_header(("Location", "/login")).finish());
        };
        match app_data.auth.check_code(code.expect("i am checking before")).await {
            Ok(channel) => {
                session.insert::<String>("twitch_channel", channel.clone())?;
                channel},
            Err(err) => {
                return Ok(actix_web::HttpResponse::TemporaryRedirect().insert_header(("Location", "/login")).finish());
            }}
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

    let secret = Key::generate();

    let twitch_guy_addr = TwitchGuy::new(server_addr.clone()).start();
    let app_data = AppData {
        broker_addr,
        twitch_guy_addr,
        server_addr,
        auth,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret.clone(),
            ))
            .app_data(app_data.clone())
            .service(index)
    })
    .bind(("0.0.0.0", 3030))?
    .run()
    .await
}
