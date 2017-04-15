use super::discord;
use discord::Discord;
use discord::model::{Event, Message};

pub mod command;

use self::command::Command;

pub struct LilBot<'a> {
    discord: Discord,
    commands: Vec<Box<Command<'a>>>,
}

impl<'a> LilBot<'a> {
    pub fn new(token: &str) -> LilBot {
        LilBot {
            discord:  Discord::from_bot_token(token).expect("invalid discord token!"),
            commands: Vec::new(),
        }
    }

    pub fn default(token: &str) -> LilBot {
        LilBot {
            discord:  Discord::from_bot_token(token).expect("invalid discord token!"),
            commands: vec![
                Box::new(command::Hello {}),
            ],
        }
    }

    pub fn discord(&self) -> &Discord {
        &self.discord
    }

    pub fn wake(&self) {
        let (mut conn, _) = self.discord.connect().expect("angery, failed connection!");
        loop {
            match conn.recv_event() {
                Ok(Event::MessageCreate(message)) => {
                    let parts: Vec<&str> = message.content.split(" ").collect();
                    for cmd in &self.commands {
                        if cmd.trigger() == parts[0] {
                            cmd.execute(&self, &message, message.content[..parts[0].len()].split(" ").collect());
                        }
                    }
                },

                Err(err) => println!("got error: {:?}", err),
                _        => (),
            }
        }
    }
}