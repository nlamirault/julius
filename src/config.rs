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

use std::fs::File;
use std::io::prelude::*;

use toml::{Parser, Value};
use toml;


#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct IrcConfig {
    pub server: String,
    pub channels: Vec<String>,
    pub username: String,
}

impl IrcConfig {
    pub fn new() -> IrcConfig {
        IrcConfig {
            channels: vec!["rust".to_owned()],
            server: "irc.freenode.net".to_owned(),
            username: "juliusbot".to_owned(),
        }
    }
}


#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct BotConfig {
    pub loglevel: String,
    pub irc: IrcConfig,
}

impl BotConfig {
    pub fn new() -> BotConfig {
        BotConfig {
            loglevel: "info".to_owned(),
            irc: IrcConfig::new(),
        }
    }

    pub fn parse(path: String) -> BotConfig {
        let mut config_toml = String::new();

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                error!("Could not find config file, using default!");
                return BotConfig::new();
            }
        };

        file.read_to_string(&mut config_toml)
            .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

        let mut parser = Parser::new(&config_toml);
        let toml = parser.parse();

        if toml.is_none() {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                         path,
                         loline,
                         locol,
                         hiline,
                         hicol,
                         err.desc);
            }
            panic!("Exiting server");
        }

        let config = Value::Table(toml.unwrap());
        toml::decode(config).unwrap()

    }
}
