use super::{LilBot, Message};


pub trait Command<'a> {
    fn execute(&self, bot: &LilBot, message: &Message, args: Vec<&str>);
    fn trigger(&self) -> &'a str;
}

pub struct Hello {}

impl<'a> Command<'a> for Hello {
    fn execute(&self, bot: &LilBot, message: &Message, args: Vec<&str>) {
        bot.discord().send_message(message.channel_id, "lil bot!", "", false);
    }

    fn trigger(&self) -> &'a str {
        "!hello"
    }
}