mod auth;
mod database;
mod game;
mod message_broker;
mod server;
mod twitch;
mod websocket;

use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_multipart::form::text::Text;
use actix_multipart::form::{bytes, MultipartForm};
use actix_web::{
    get, post, web, App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use anyhow::{Context, Result};
use auth::Auth;
use database::DataBase;
use serde::Deserialize;
use server::Server;
use twitch::{Channel, TwitchGuy};

use crate::database::Image;
use crate::{message_broker::MessageBroker, websocket::WebsocketGuy};

use actix_session::storage::CookieSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::Key;

#[derive(Deserialize)]
struct QueryParameter {
    code: Option<String>,
}

#[get("/login")]
async fn login(
    query: web::Query<QueryParameter>,
    req: HttpRequest,
    session: Session,
) -> Result<HttpResponse, Error> {
    println!("i am runnning");
    let app_data: &AppData = req
        .app_data()
        .expect("app data should always exist i think");

    let code = query.code.clone();

    if code == None {
        return Ok(actix_web::HttpResponse::TemporaryRedirect()
            .insert_header(("Location", "/login"))
            .finish());
    };
    match app_data
        .auth
        .check_code(code.expect("i am checking before"))
        .await
    {
        Ok(user_data) => {
            session.insert::<String>("twitch_channel", user_data.channel.clone())?;
            session.insert::<String>("id", user_data.id)?;
            user_data.channel
        }
        Err(_) => {
            // not sure if this is the right status code
            return Ok(actix_web::HttpResponse::TemporaryRedirect()
                .insert_header(("Location", "/login"))
                .finish());
        }
    };

    Ok(HttpResponse::Ok()
        .header("Access-Control-Allow-Credentials", "true")
        .finish())
}

#[get("/ws")]
async fn index(
    req: HttpRequest,
    stream: web::Payload,
    session: Session,
) -> Result<HttpResponse, Error> {
    let app_data = req
        .app_data::<AppData>()
        .expect("AppData should always exist i think")
        .clone();

    let channel = match session.get::<String>("twitch_channel") {
        Ok(Some(channel)) => channel,
        _ => {
            return Ok(actix_web::HttpResponse::TemporaryRedirect()
                .insert_header(("Location", "/login"))
                .finish())
        }
    };
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
    db: DataBase,
}

#[derive(Debug, MultipartForm)]
pub struct ImageForm {
    pub description: Text<String>,
    pub tags: Text<String>,
    #[multipart(limit = "128 MiB", rename = "image")]
    pub files: Vec<bytes::Bytes>,
    pub year: Text<u16>,
}
#[post("/image")]
async fn add_image(
    MultipartForm(form): MultipartForm<ImageForm>,
    session: Session,
    req: HttpRequest,
) -> HttpResponse {
    // TODO:
    // get id of user somehow
    // insert into database

    let app_data = req.app_data::<AppData>().expect("app data to exist");

    let id = match session.get::<String>("id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::InternalServerError().finish(),
    };

    app_data.db.insert_image(Image::from_form(form, id)).await;
    // println!("{:?}",payload.description.as_str());

    HttpResponse::Ok().body("ty for adding an image")
}

#[derive(Deserialize)]
struct ImageQuery {
    image: String,
}

#[get("image")]
async fn get_image(
    req: HttpRequest,
    image: web::Query<ImageQuery>,
    session: Session,
) -> HttpResponse {
    // match session.get::<String>("id") {
    //     Ok(Some(id)) => (),
    //     _ => return HttpResponse::InternalServerError().finish(),
    // };
    let image = req
        .app_data::<AppData>()
        .expect("app_data should exist")
        .db
        .get_image_bytes_by_id(&image.0.image)
        .await;
    if let Ok(Some(image_raw)) = image {
        return HttpResponse::Ok().content_type("image/png").body(image_raw);
    };
    todo!();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let broker_addr = message_broker::MessageBroker::default().start();

    let server_addr = Server::new(broker_addr.clone()).start();

    let secret = Key::generate();

    let twitch_guy_addr = TwitchGuy::new(server_addr.clone()).start();

    let db = DataBase::new().await;

    let auth = Auth::new(db.clone());

    let app_data = AppData {
        broker_addr,
        twitch_guy_addr,
        server_addr,
        auth,
        db,
    };

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_header();
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret.clone(),
            ))
            .wrap(cors)
            .app_data(app_data.clone())
            .service(index)
            .service(add_image)
            .service(get_image)
            .service(login)
    })
    .bind(("0.0.0.0", 3030))?
    .run()
    .await
}
