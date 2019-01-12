#[macro_use]
extern crate log;
#[macro_use]
extern crate serenity;

extern crate env_logger;
extern crate kankyo;
extern crate rand;

mod commands;

use std::{collections::HashSet, env};

use serenity::{
    framework::StandardFramework,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
    http,
};

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

fn main() {
    kankyo::load().expect("Failed to load .env file");
    env_logger::init().expect("Failed to initialize env_logger");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix("."))
        .command("d", |c| c.cmd(commands::dice::d)));

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
