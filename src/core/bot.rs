use std::thread;
use websocket::header::Authorization;

use crate::utils::net::responses::gateway::GatewayInfos;

const DISCORD_API_URL: &str = "https://discord.com/api/v9";

pub struct Bot<> {
    token: String,
    on_message: Box<dyn FnMut() -> bool>,
}
impl Bot {
    pub fn new(token: &str, on_message: Box<dyn FnMut() -> bool>) -> Self {
        Self {
            token: String::from(token),
            on_message: on_message,
        }
    }
    pub fn get_token(&self) -> &str {
        &self.token
    }
    pub fn connect(&self) {
        // thread::spawn(|| { // Loop with buffer for sending user request, like set_presence(ONLINE) and get all event for fire related functions
        //     let mut headers = websocket::header::Headers::new();
        //     headers.set(Authorization(format!("Bot {}", self.get_token()).to_owned()));        
        // });
    }

    // Utils functions for intern use only
    pub fn get_gateway_infos(&self) -> GatewayInfos {
        let gateway_infos = reqwest::blocking::Client::new()
            .get(format!("{}/gateway/bot", DISCORD_API_URL))
            .header("Authorization", format!("Bot {}", self.get_token()))
            .send()
            .map_or_else(|e| {
                println!("Unable to get gateway url");
                panic!(e);
            }, |res| {
                res.json::<GatewayInfos>().unwrap()
            });

            return gateway_infos;

    }
}