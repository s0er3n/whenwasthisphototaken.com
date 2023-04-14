use std::{collections::HashMap, fs};

use actix::{Actor, Addr, Context, Handler, Message};

use rand::Rng;
use regex::Regex;
use serde::Serialize;
use twitch_irc::message::PrivmsgMessage;

use crate::message_broker::{BrokerMessage, MessageBroker};

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
    // i think not necessary i sorted in frontend
    let mut res: Vec<(String, f64)> = result.into_iter().collect();
    res.sort_by_key(|k| k.1 as usize);
    res
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
        // there are 123 years
        let mut res = vec![0; 124];

        for guess in self.guesses.values() {
            // starting at 1900
            let index = guess - 1900;
            res[index as usize] += 1
        }

        return res;
    }
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

pub struct Game {
    state: GameState,
    images: Vec<Image>,
    round_number: u8,
    broker_addr: Addr<MessageBroker>,
}

impl Game {
    pub fn new(broker_addr: Addr<MessageBroker>, channel: String) -> Self {
        let new = Self {
            images: vec![
                Image::random_image(),
                Image::random_image(),
                Image::random_image(),
                Image::random_image(),
                Image::random_image(),
            ],
            state: GameState::Image,
            round_number: 0,
            broker_addr,
        };
        new.broker_addr.do_send(BrokerMessage {
            payload: new.to_message(),
            channel,
        });
        new
    }
}

impl Game {
    fn add_guess(&mut self, user: String, year: u16) -> Result<(), ()> {
        if let Some(image) = self.images.get_mut(self.round_number as usize) {
            image.guesses.insert(user.clone(), year);
            let score = calculate_score(image.result.into(), year.into());
            image.scores.insert(user, score);
            Ok(())
        } else {
            println!("couldnt get images as mut");
            Err(())
        }
    }

    fn reset(&mut self) {
        self.round_number = 0;
        self.images = vec![
            Image::random_image(),
            Image::random_image(),
            Image::random_image(),
            Image::random_image(),
            Image::random_image(),
        ];
        self.state = GameState::Image;
    }

    fn next(&mut self) {
        match self.state {
            GameState::AfterImage => match self.images.get(self.round_number as usize + 1) {
                Some(_) => {
                    self.state = GameState::Image;
                    self.round_number += 1;
                }
                None => {
                    self.state = GameState::Results;
                }
            },
            GameState::Image => {
                self.state = GameState::AfterImage;
            }
            GameState::Results => self.reset(),
        };
    }

    fn to_message(&self) -> String {
        let message = match self.state {
            GameState::Image => StateMsg::Image {
                url: self.images[self.round_number as usize].url.clone(),
                guesses: self.images[self.round_number as usize].create_vec_guesses(),
                pos: self.round_number as usize + 1,
                len: self.images.len(),
            },
            GameState::AfterImage => StateMsg::AfterImage {
                url: self.images[self.round_number as usize].url.clone(),
                guesses: self.images[self.round_number as usize].create_vec_guesses(),
                description: self.images[self.round_number as usize].description.clone(),
                result: self.images[self.round_number as usize].result.clone(),
                scores: self.images[self.round_number as usize]
                    .scores
                    .clone()
                    .into_iter()
                    // TODO: sort
                    .collect(),
                pos: self.round_number as usize + 1,
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

impl Actor for Game {
    type Context = Context<Self>;
}

pub struct TwitchMsg {
    msg: String,
    author: String,
    author_id: String,
    channel: String,
}

impl Message for TwitchMsg {
    type Result = ();
}

impl TwitchMsg {
    fn find_year(&self) -> Option<u16> {
        let re = Regex::new(r"\b(19\d{2}|20[0|1]\d|202[0-2])\b").unwrap();
        if let Some(cap) = re.captures(&self.msg) {
            if let Some(year) = cap.get(0) {
                return year.as_str().parse::<u16>().ok();
            }
        }
        None
    }
}

impl From<PrivmsgMessage> for TwitchMsg {
    fn from(msg: PrivmsgMessage) -> Self {
        Self {
            msg: msg.message_text,
            author: msg.sender.name,
            author_id: msg.sender.id,
            channel: msg.channel_login,
        }
    }
}

impl Handler<TwitchMsg> for Game {
    type Result = ();

    fn handle(&mut self, msg: TwitchMsg, ctx: &mut Self::Context) -> Self::Result {
        if msg.channel == msg.author.to_lowercase() && msg.msg.starts_with("!next") {
            self.next();
            self.broker_addr.do_send(BrokerMessage {
                channel: msg.channel,
                payload: self.to_message(),
            });
            return ();
        }
        if let (&GameState::Image, Some(year)) = (&self.state, msg.find_year()) {
            {
                let _ = self.add_guess(msg.author, year);
                self.broker_addr.do_send(BrokerMessage {
                    channel: msg.channel,
                    payload: self.to_message(),
                });
            }
        };
    }
}
