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

use irc::client::data::{Command, Config, Message};
use irc::client::server::{IrcServer, Server};
use irc::client::server::utils::ServerExt;


pub struct Bot {
    server: IrcServer,
}

impl Bot {
    pub fn new_from_config(config: Config) -> Bot {
        let server = IrcServer::from_config(config).unwrap();
        Bot { server: server }
    }

    pub fn run(&self) {
        self.server.identify().unwrap();
        for message in self.server.iter() {
            match message {
                Ok(ok_msg) => self.handle_message(&ok_msg),
                Err(e) => error!("error: {:?}", e),
            }
        }
    }

    fn quit(&self) {
        self.server.send_quit("in tartiflette we trust").unwrap();
    }


    fn handle_message(&self, msg: &Message) {
        debug!("Irc message{:?}", msg);
        match msg.command {
            Command::PRIVMSG(ref target, ref msg) => {
                if msg.contains("!version") {
                    self.server.send_privmsg(target, "0.1.0").unwrap();
                } else if msg.contains("!quit") {
                    self.quit();
                } else if msg.contains("!help") {
                    self.server.send_notice(target, r#"Julius -- An IRC bot"#).unwrap();
                    self.server.send_notice(target, r#"Commands"#).unwrap();
                    self.server.send_notice(target, r#"   !help : show this help"#).unwrap();
                    self.server
                        .send_notice(target, r#"   !version : show the bot version"#)
                        .unwrap();
                }
            }
            _ => (),
        }
    }
}
