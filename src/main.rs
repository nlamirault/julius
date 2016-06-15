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


extern crate irc;

use std::default::Default;
use irc::client::prelude::*;

mod version;


fn main() {
    let config = Config {
        nickname: Some(format!("juliusbot")),
        alt_nicks: Some(vec![format!("juliusbot_"), format!("juliusbot__")]),
        server: Some(format!("irc.freenode.net")),
        channels: Some(vec![format!("#perave")]),
        .. Default::default()
    };
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        print!("{}", message);
        match message.command {
            Command::PRIVMSG(ref target, ref msg) =>
                if msg.contains("!version") {
                    server.send_privmsg(target, &version::version()).unwrap();
                } else if msg.contains("!help") {
                    server.send_privmsg(target, "Julius -- An IRC bot").unwrap();
                },
            _ => (),
        }
    }
}
