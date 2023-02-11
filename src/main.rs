use std::collections::HashMap;
use std::sync::Arc;

use serde::Serialize;
use serde_json;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::RwLock;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use futures_util::{SinkExt, StreamExt};
use warp::ws::{Message, WebSocket};
use warp::Filter;

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
            image.guesses.insert(user, year);
            Ok(())
        } else {
            println!("couldnt get images as mut");
            Err(())
        }
    }

    fn next(&mut self) -> Option<Self> {
        match self.state {
            GameState::AfterImage => {
                self.state = GameState::Image;
                self.i += 1;
                return None;
            }
            GameState::Image => match self.images.get(self.i as usize + 1) {
                Some(_) => {
                    self.state = GameState::AfterImage;
                }
                _ => {
                    self.state = GameState::Results;
                }
            },
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
                pos: self.i as usize + 1,
                len: self.images.len(),
            },
            GameState::Results => StateMsg::Results {},
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
        // TODO: add scoreboard or winners
    },
    Results {
        // TODO: scoreboard winner etc
    },
}

#[derive(Debug)]
struct Image {
    url: String,
    result: u16,
    description: String,
    // string -> sender
    guesses: HashMap<String, u16>,
}

impl Image {
    fn random_image() -> Self {
        Self {
            url: "not implemented".to_owned(),
            result: 1999,
            description: "description".to_owned(),
            guesses: HashMap::new(),
        }
    }
    fn create_vec_guesses(&self) -> Vec<u16> {
        // 123 years
        let mut res = vec![0; 123];

        for guess in self.guesses.values() {
            // starting at 1900
            let index = guess - 1900;
            res[index as usize] += 1
        }

        return res;
    }
}

async fn handle_year_msg(message: &PrivmsgMessage, server: &mut Server, sub: Arc<RwLock<Sub>>) {
    if message.message_text.len() >= 4 {
        if let Ok(year) = message.message_text.parse::<u16>() {
            if 1900 <= year && year <= 2023 {
                if let Some(game) = server.games.get_mut(&message.channel_login) {
                    if let GameState::Image = game.state {
                        let res = game.add_guess(message.sender.name.clone(), year);
                        if res.is_err() {
                            println!("couldnt guess");
                        }
                        dbg!(&year);
                        sub.read()
                            .await
                            .send_message(game.to_message(), message.channel_login.clone());
                        dbg!(&year);
                    }
                }
            }
        }
    }
}
async fn handle_next(message: &PrivmsgMessage, server: &mut Server, sub: Arc<RwLock<Sub>>) {
    if message.message_text.starts_with("!next")
        && message.channel_login == message.sender.name.to_lowercase()
    {
        if let Some(game) = server.games.get_mut(&message.channel_login) {
            if let Some(game) = game.next() {
                server.games.insert(message.channel_login.clone(), game);
                sub.read().await.send_message(
                    server
                        .games
                        .get(&message.channel_login)
                        .unwrap()
                        .to_message(),
                    message.channel_login.clone(),
                );
            }
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
            dbg!("sending stuff");
            rx.send(msg.clone()).unwrap();
        });
    }
}

#[tokio::main]
pub async fn main() {
    // default configuration is to join chat as anonymous.
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let mut server = Server::new();

    let sub = Sub::new(client);
    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    // so in this simple case where the channel name is hardcoded we can ignore the potential
    // error with `unwrap`.
    // client.join("soeren_______".to_owned()).unwrap();

    // server.games.insert("soeren_______".to_owned(), Game::new());
    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    {
        let sub = sub.clone();
        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                if let twitch_irc::message::ServerMessage::Privmsg(message) = message {
                    dbg!(&message);
                    handle_year_msg(&message, &mut server, sub.clone()).await;
                    handle_next(&message, &mut server, sub.clone()).await;
                }
            }
        });
    }

    let sub = warp::any().map(move || sub.clone());
    let chat = warp::path("ws")
        .and(warp::path::param::<String>())
        .and(warp::ws())
        .and(sub)
        .map(
            move |channel: String, ws: warp::ws::Ws, sub: Arc<RwLock<Sub>>| {
                ws.on_upgrade(move |socket| user_connected(channel, socket, sub))
            },
        );

    warp::serve(chat).run(([127, 0, 0, 1], 3030)).await;
    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
}

async fn user_connected(channel: String, socket: WebSocket, sub: Arc<RwLock<Sub>>) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    {
        sub.write().await.subscribe(channel, tx);
    }
    let (mut user_ws_tx, user_ws_rx) = socket.split();

    dbg!("user connected");
    let join_handle = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            user_ws_tx.send(Message::text(msg)).await.unwrap();
        }
    });
}
