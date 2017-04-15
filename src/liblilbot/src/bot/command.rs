use super::{LilBot, Message};

#[derive(Debug, Clone)]
pub enum Trigger {
    Only(String),
    Args(String),
}

pub trait Command {
    fn execute(&self, bot: &LilBot, message: &Message, args: Vec<&str>);
    fn triggers(&self) -> Vec<Trigger>;
}

pub struct DisplayContent {
    triggers: Vec<Trigger>,
    content:  String,
}

impl DisplayContent {
    pub fn new(triggers: Vec<Trigger>, content: String) -> DisplayContent {
        DisplayContent {
            triggers: triggers,
            content:  content,
        }
    }
}

impl Command for DisplayContent {
    fn execute(&self, bot: &LilBot, message: &Message, args: Vec<&str>) {
        println!("{:?} => {:#?}", self.triggers, args);

        bot.discord().send_message(message.channel_id, &self.content, "", false);
    }

    fn triggers(&self) -> Vec<Trigger> {
        self.triggers.clone()
    }
}