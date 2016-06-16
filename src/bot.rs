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

use plugins::plugin::Plugin;
use plugins::help::HelpPlugin;

pub struct Bot {
    server: IrcServer,
    plugins: Vec<Box<Plugin>>,
}

impl Bot {
    pub fn new_from_config(config: Config) -> Bot {
        let server = IrcServer::from_config(config).unwrap();
        let help_plugin = HelpPlugin::new();
        let mut plugins: Vec<Box<Plugin>> = Vec::new();
        plugins.push(Box::new(help_plugin));
        Bot {
            server: server,
            plugins: plugins,
        }
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

    fn do_plugin(&self, text: &str, target: &str, msg: String) {
        if let Some(index) = self.plugins
            .iter()
            .position(|p| p.get_command() == text) {
            self.plugins[index].handle_message(self.server.clone(), target, msg.to_owned());
        } else {
            self.server
                .send_notice(target, r#"Invalid command. Try !help."#)
                .unwrap();
        }
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
                    self.do_plugin("!help", target, msg.to_owned())
                } else {
                }
            }
            _ => (),
        }
    }
}
