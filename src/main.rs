// Copyright (C) 2016 Nicolas Lamirault <nicolas.lamirault@gmail.com>

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate irc;
extern crate clap;
extern crate toml;
extern crate rustc_serialize;

use std::default::Default;
use clap::{Arg, App, SubCommand};
use irc::client::prelude::*;

use config::BotConfig;

mod bot;
mod config;

fn cmd_irc(config: BotConfig) {
    let config = Config {
        nickname: Some(config.irc.username),
        alt_nicks: Some(vec![format!("juliusbot_"), format!("juliusbot__")]),
        server: Some(config.irc.server),
        // channels: Some(vec![format!("#perave")]),
        channels: Some(config.irc.channels),
        ..Default::default()
    };
    let bot = bot::Bot::new_from_config(config);
    bot.run();
}


fn main() {
    env_logger::init().unwrap();
    info!("[julius] Starting");

    let matches = App::new("Julius")
        .version("0.1.0")
        .author("Nicolas Lamirault <nicolas.lamirault@gmail.com>")
        .about("An IRC bot")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .required(false)
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("irc").about("Connect IRC"))
        .get_matches();

    let path: String = matches.value_of("config").unwrap_or("julius.toml").parse().unwrap();
    info!("[julius] Configuration file: {}", path);
    let config = BotConfig::parse(path);
    info!("[julius] Starting using configuration: {:?}", config);

    match matches.subcommand() {
        ("irc", Some(_)) => cmd_irc(config),
        _ => {
            println!("Unknown subcommand (try help)");
        }
    }


}
