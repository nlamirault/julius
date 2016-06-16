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

use irc::client::server::IrcServer;
use irc::client::server::utils::ServerExt;

use plugins::plugin::Plugin;


pub struct HelpPlugin {
    command: String,
}

impl HelpPlugin {
    pub fn new() -> HelpPlugin {
        HelpPlugin { command: "!help".to_owned() }
    }
}

impl Plugin for HelpPlugin {
    fn get_command(&self) -> String {
        self.command.to_owned()
    }

    fn handle_message(&self, server: IrcServer, target: &str, msg: String) {
        server.send_notice(target, r#"Julius -- An IRC bot"#).unwrap();
        server.send_notice(target, r#"Commands"#).unwrap();
        server.send_notice(target, r#"   !help : show this help"#).unwrap();
        server.send_notice(target, r#"   !version : show the bot version"#)
            .unwrap();
    }
}
