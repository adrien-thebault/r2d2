#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate actix;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

extern crate bytes;
extern crate chrono;
extern crate fern;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate slack;
extern crate slack_api;
extern crate tokio_codec;

#[macro_use]
mod config;
mod actors;
mod logger;
mod messages;

use logger::Logger;
use messages::Envelope;

use actix::prelude::*;
use slack::RtmClient;
use std::{fs, str::FromStr};

const APP_NAME: &str = "r2d2";

lazy_static! {
    static ref CONFIG_FILE: String = format!("{}.json", APP_NAME);
    pub static ref CONFIG: config::Config = {
        match fs::read_to_string(&*CONFIG_FILE) {
            Ok(configuration) => match serde_json::from_str::<config::Config>(&configuration) {
                Ok(config) => config,
                Err(e) => {
                    println!("{}", e);
                    panic!("couldn't parse config");
                }
            },
            Err(e) => {
                println!("{}", e);
                panic!("couldn't parse config");
            }
        }
    };
}

struct Handler(Addr<actors::SlackClient>);

impl slack::EventHandler for Handler {
    fn on_event(&mut self, _cli: &RtmClient, event: slack::Event) {
        debug!("event received : {:?}", event);
        self.0.do_send::<Envelope>(event.into());
    }

    fn on_close(&mut self, _cli: &RtmClient) {
        info!("{} disconnected", APP_NAME);
    }

    fn on_connect(&mut self, _cli: &RtmClient) {
        info!("{} connected", APP_NAME);
    }
}

fn main() {
    init_logger();
    System::run(|| {
        match RtmClient::login(config!(api_key)) {
            Ok(rtm) => {
                let slack_client = actors::SlackClient::new(rtm.sender().clone()).start();

                for recipient in vec![
                    actors::Admin::new(slack_client.clone()).start().recipient(),
                    actors::Misc::new(slack_client.clone()).start().recipient(),
                    actors::Pendu::new(slack_client.clone()).start().recipient(),
                    actors::Quiz::new(slack_client.clone()).start().recipient(),
                    actors::Stats::new(slack_client.clone()).start().recipient(),
                ] {
                    slack_client.do_send(messages::RegisterRecipient::new(recipient));
                }

                let mut handler = Handler(slack_client);

                std::thread::spawn(move || {
                    if let Err(e) = rtm.run(&mut handler) {
                        error!("{}", e);
                    }
                    System::current().stop();
                });
            }
            Err(e) => {
                error!("{}", e);
                System::current().stop();
            }
        };
    });
}

fn init_logger() {
    let log_lvl = match log::LevelFilter::from_str(&config!(verbosity)) {
        Ok(log_lvl) => log_lvl,
        Err(_e) => log::LevelFilter::Info,
    };

    Logger::init(log_lvl);
}
