use actix::{Actor, Addr, Context, Handler, Message};
use twitch_irc::{
    login::StaticLoginCredentials,
    message::ServerMessage,
    transport::tcp::{TCPTransport, TLS},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

use crate::server::Server;

pub struct TwitchMessage(pub ServerMessage);

impl Message for TwitchMessage {
    type Result = ();
}

pub enum Channel {
    Join(String),
    Leave(String),
}

impl Message for Channel {
    type Result = ();
}

pub struct TwitchGuy {
    // not used yet but maybe later for sending messages
    client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
}

impl Handler<Channel> for TwitchGuy {
    type Result = ();

    fn handle(&mut self, msg: Channel, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Channel::Join(channel) => self.client.join(channel),
            Channel::Leave(channel) => todo!(),
        };
    }
}

impl TwitchGuy {
    pub fn new(server_address: Addr<Server>) -> Self {
        let config = ClientConfig::default();
        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
        let _ = actix_web::rt::spawn(async move {
            while let Some(msg) = incoming_messages.recv().await {
                server_address.do_send(TwitchMessage(msg))
            }
        });

        Self { client }
    }
}

impl Actor for TwitchGuy {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("TwitchGuy is born");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("TwitchGuy died");
    }
}
