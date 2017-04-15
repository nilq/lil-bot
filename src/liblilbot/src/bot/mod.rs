use super::discord;
use discord::Discord;
use discord::model::{Event, Message};

pub mod command;

use self::command::Command;

pub struct LilBot {
    discord: Discord,
    commands: Vec<Box<Command>>,
}

impl LilBot {
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
                Box::new(command::DisplayContent::new(
                    vec![command::Trigger::Only("!ping".to_string())], "lil-boat!".to_string())
                ),
                Box::new(command::DisplayContent::new(
                    vec![command::Trigger::Only("!help".to_string())], "yeye hello :thinking:".to_string())
                ),
                Box::new(command::DisplayContent::new(
                    vec![command::Trigger::Args("!pay".to_string())], "printed secret things >:(".to_string())
                ),
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
                        for trigger in cmd.triggers() {
                            match trigger {
                                command::Trigger::Only(c) => if c == parts[0].to_owned() {
                                    if parts.len() > 1 {
                                        break
                                    }
                                    cmd.execute(&self, &message, vec![]);
                                },

                                command::Trigger::Args(c) => if c == parts[0].to_owned() {
                                    cmd.execute(&self, &message, message.content.split(" ").collect());
                                },
                            }
                        }
                    }
                },

                Err(err) => println!("got error: {:?}", err),
                _        => (),
            }
        }
    }
}