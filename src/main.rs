use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use rand::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::RwLock;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use futures_util::{SinkExt, StreamExt};
use std::f64::consts::E;
use warp::ws::{Message, WebSocket};
use warp::Filter;

static MIN_YEAR: f64 = 1900.;
static MAX_YEAR: f64 = 2023.;

fn calculate_score(real_year: f64, guessed_year: f64) -> f64 {
    let diff = f64::abs(real_year - guessed_year);
    let decay = f64::exp(-diff / ((MAX_YEAR - MIN_YEAR) * 2.0)); // add exponential decay term
    5000.0 * decay * f64::exp(-diff / (MAX_YEAR - MIN_YEAR))
}
fn combine_hash_maps(maps: &Vec<HashMap<String, f64>>) -> Vec<(String, f64)> {
    let mut result = HashMap::new();
    for map in maps {
        for (key, value) in map {
            *result.entry(key.to_owned()).or_default() += value.clone();
        }
    }
    // TODO: sort
    let mut res: Vec<(String, f64)> = result.into_iter().collect();
    res.sort_by_key(|k| k.1 as usize);
    res
}

struct Server {
    // string -> channel
    games: HashMap<String, Game>,
}

impl Server {
    fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }
}

#[derive(Debug)]
enum GameState {
    Image,
    AfterImage,
    Results,
}
#[derive(Debug)]
struct Game {
    state: GameState,
    images: Vec<Image>,
    i: u8,
}

impl Game {
    fn new() -> Self {
        Self {
            images: vec![
                Image::random_image(),
                Image::random_image(),
                Image::random_image(),
                Image::random_image(),
                Image::random_image(),
            ],
            state: GameState::Image,
            i: 0,
        }
    }

    fn add_guess(&mut self, user: String, year: u16) -> Result<(), ()> {
        if let Some(image) = self.images.get_mut(self.i as usize) {
            image.guesses.insert(user.clone(), year);
            let score = calculate_score(image.result.into(), year.into());
            image.scores.insert(user, score);
            Ok(())
        } else {
            println!("couldnt get images as mut");
            Err(())
        }
    }

    fn next(&mut self) -> Option<Self> {
        match self.state {
            GameState::AfterImage => match self.images.get(self.i as usize + 1) {
                Some(_) => {
                    self.state = GameState::Image;
                    self.i += 1;
                }
                None => {
                    self.state = GameState::Results;
                }
            },
            GameState::Image => {
                self.state = GameState::AfterImage;
            }
            GameState::Results => return Some(Self::new()),
        }
        None
    }

    fn to_message(&self) -> String {
        let message = match self.state {
            GameState::Image => StateMsg::Image {
                url: self.images[self.i as usize].url.clone(),
                guesses: self.images[self.i as usize].create_vec_guesses(),
                pos: self.i as usize + 1,
                len: self.images.len(),
            },
            GameState::AfterImage => StateMsg::AfterImage {
                url: self.images[self.i as usize].url.clone(),
                guesses: self.images[self.i as usize].create_vec_guesses(),
                description: self.images[self.i as usize].description.clone(),
                result: self.images[self.i as usize].result.clone(),
                scores: self.images[self.i as usize]
                    .scores
                    .clone()
                    .into_iter()
                    // TODO: sort
                    .collect(),
                pos: self.i as usize + 1,
                len: self.images.len(),
            },
            GameState::Results => StateMsg::Results {
                scores: combine_hash_maps(
                    &self
                        .images
                        .iter()
                        // probably avoidable clone
                        .map(|image| image.scores.clone())
                        .collect(),
                ),
            },
        };
        serde_json::to_string(&message).unwrap()
    }
}

#[derive(Serialize)]
enum StateMsg {
    Image {
        url: String,
        guesses: Vec<u16>,
        pos: usize,
        len: usize,
    },
    AfterImage {
        url: String,
        guesses: Vec<u16>,
        description: String,
        result: u16,
        pos: usize,
        len: usize,
        scores: Vec<(String, f64)>,
        // TODO: add scoreboard or winners
    },
    Results {
        // TODO: scoreboard winner etc
        scores: Vec<(String, f64)>,
    },
}

#[derive(Debug)]
struct Image {
    url: String,
    result: u16,
    description: String,
    // string -> sender
    guesses: HashMap<String, u16>,
    scores: HashMap<String, f64>,
}

impl Image {
    fn random_image() -> Self {
        let (result, url) = get_random_image();
        Self {
            url,
            result,
            description: "description".to_owned(),
            guesses: HashMap::new(),
            scores: HashMap::new(),
        }
    }
    fn create_vec_guesses(&self) -> Vec<u16> {
        // 123 years
        let mut res = vec![0; 124];

        for guess in self.guesses.values() {
            // starting at 1900
            let index = guess - 1900;
            res[index as usize] += 1
        }

        return res;
    }
}

async fn handle_year_msg(
    message: &PrivmsgMessage,
    server: Arc<RwLock<Server>>,
    sub: Arc<RwLock<Sub>>,
) {
    if message.message_text.len() >= 4 {
        if let Ok(year) = message.message_text.parse::<u16>() {
            if 1900 <= year && year <= 2023 {
                if let Some(game) = server.write().await.games.get_mut(&message.channel_login) {
                    if let GameState::Image = game.state {
                        let res = game.add_guess(message.sender.name.clone(), year);
                        if res.is_err() {
                            println!("couldnt guess");
                        }
                        sub.read()
                            .await
                            .send_message(game.to_message(), message.channel_login.clone());
                    }
                }
            }
        }
    }
}
async fn handle_next(message: &PrivmsgMessage, server: Arc<RwLock<Server>>, sub: Arc<RwLock<Sub>>) {
    if message.message_text.starts_with("!next")
        && message.channel_login == message.sender.name.to_lowercase()
    {
        let mut server = server.write().await;
        if let Some(game) = server.games.get_mut(&message.channel_login) {
            if let Some(game) = game.next() {
                server.games.insert(message.channel_login.clone(), game);
            }
            let msg = server
                .games
                .get(&message.channel_login)
                .unwrap()
                .to_message();

            sub.read()
                .await
                .send_message(msg, message.channel_login.clone());
        }
    }
}

struct Sub {
    listeners: HashMap<String, Vec<UnboundedSender<String>>>,
    client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
}
impl Sub {
    fn new(
        client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
    ) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            listeners: HashMap::new(),
            client,
        }))
    }

    fn subscribe(&mut self, channel: String, tx: UnboundedSender<String>) {
        self.listeners
            .entry(channel.clone())
            .or_insert(vec![])
            .push(tx);
        // TODO: only join first time not sure if it matters
        let _ = self.client.join(channel);
    }

    fn send_message(&self, msg: String, channel: String) {
        self.listeners.get(&channel).unwrap().iter().for_each(|rx| {
            let _ = rx.send(msg.clone());
        });
    }
}

#[tokio::main]
pub async fn main() {
    // default configuration is to join chat as anonymous.
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let mut server = Arc::new(RwLock::new(Server::new()));

    let sub = Sub::new(client);
    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    // so in this simple case where the channel name is hardcoded we can ignore the potential
    // error with `unwrap`.
    // client.join("soeren_______".to_owned()).unwrap();

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    {
        let sub = sub.clone();
        let server = server.clone();
        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                if let twitch_irc::message::ServerMessage::Privmsg(message) = message {
                    handle_next(&message, server.clone(), sub.clone()).await;
                    handle_year_msg(&message, server.clone(), sub.clone()).await;
                }
            }
        });
    }

    let sub = warp::any().map(move || sub.clone());
    let server = warp::any().map(move || server.clone());
    let chat = warp::path("ws")
        .and(warp::path::param::<String>())
        .and(warp::ws())
        .and(sub)
        .and(server)
        .map(
            move |channel: String,
                  ws: warp::ws::Ws,
                  sub: Arc<RwLock<Sub>>,
                  server: Arc<RwLock<Server>>| {
                ws.on_upgrade(move |socket| user_connected(channel, socket, sub, server))
            },
        );

    warp::serve(chat).run(([0, 0, 0, 0], 3030)).await;
    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
}

async fn user_connected(
    channel: String,
    socket: WebSocket,
    sub: Arc<RwLock<Sub>>,
    server: Arc<RwLock<Server>>,
) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let (mut user_ws_tx, user_ws_rx) = socket.split();
    {
        sub.write().await.subscribe(channel.clone(), tx);
        let msg = server
            .write()
            .await
            .games
            .entry(channel)
            .or_insert_with(|| Game::new())
            .to_message();

        let _ = user_ws_tx.send(Message::text(msg)).await;
    }

    dbg!("user connected");
    let join_handle = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            dbg!(&msg);
            let _ = user_ws_tx.send(Message::text(msg)).await;
        }
    });
}
fn get_random_image() -> (u16, String) {
    let file = fs::read_to_string("./images.txt").unwrap();
    let year_photos: std::collections::HashMap<u16, Vec<String>> =
        serde_json::from_str(&file).unwrap();

    let mut rng = rand::thread_rng();
    let random_year: u16 = rng.gen_range(1900..2023);
    let photos = year_photos.get(&random_year).unwrap();
    let photo_index: usize = rng.gen_range(0..photos.len());

    let result = (
        random_year,
        format!("https://{}", photos[photo_index].clone()),
    );
    result
}
