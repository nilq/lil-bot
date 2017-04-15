extern crate liblilbot;

use std::env;

use self::liblilbot::bot;
use bot::LilBot;
use bot::command;

fn main() {
    let token = env::args().nth(1).unwrap();
    LilBot::default(&token).wake();
}
