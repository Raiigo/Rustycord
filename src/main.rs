use std::{net::TcpStream, thread, time::Duration};

use reqwest::*;
use tokio::*;
use serde::{Serialize, Deserialize};
use websocket::{ClientBuilder, Message, header::Authorization};
use native_tls::*;

#[derive(Deserialize)]
struct SessionStartLimit {
    total: u32,
    remaining: u32,
    reset_after: u32,
    max_concurrency: u32,
}

#[derive(Deserialize)]
struct GatewayInfos {
    url: String,
    shards: u8,
    session_start_limit: SessionStartLimit,
}
impl GatewayInfos {
    pub fn get_url(&self) -> &str {
        return &self.url;
    }
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {

    let response = reqwest::Client::new()
        .request(Method::GET, "https://discord.com/api/v9/gateway/bot")
        .header("Authorization", "Bot token_artifact")
        .send().await?.json::<GatewayInfos>().await;

    let gateway_infos = match response {
        Ok(gateway_infos) => gateway_infos,
        Err(e) => {
            println!("Error while deserializing the response : {}", e);
            panic!();
        }
    };

    println!("{}", gateway_infos.get_url());

    println!("Ok !");

    let mut headers = websocket::header::Headers::new();
    headers.set(Authorization("Bot token_artifact".to_owned()));

    let mut client = ClientBuilder::new(gateway_infos.get_url()).unwrap()
        .custom_headers(&headers)
        .connect_secure(Some(TlsConnector::new().unwrap())).unwrap();

    let mut text_message = match client.recv_message().unwrap() {
        websocket::OwnedMessage::Text(text) => text,
        websocket::OwnedMessage::Binary(bin) => String::new(),
        websocket::OwnedMessage::Close(close) => String::new(),
        websocket::OwnedMessage::Ping(ping) => String::new(),
        websocket::OwnedMessage::Pong(pong) => String::new(),
    };

    text_message = text_message.replace("\"t\":null,\"s\":null,", "");
    text_message.replace_range(40.., "}}");
    text_message = text_message.replace(r#"{"op":10,"d":{"heartbeat_interval":"#, "").replace("}}", "");
    
    println!("{}", text_message);

    let heartbeat_interval: u128 = text_message.parse::<u128>().unwrap();

    thread::spawn(move || -> reqwest::Result<()> {
        let mut delta = std::time::Instant::now();
        let response = client.send_message(&Message::text(r#"
        {
            "op": 2,
            "d": {
              "token": "token_artifact",
              "intents": 513,
              "properties": {
                "$os": "linux",
                "$browser": "rust_lib",
                "$device": "rust_lib"
              },
              "presence": {
                "activities": [{
                  "name": "League of Legends",
                  "type": 1
                }],
                "status": "online",
                "since": 91879201,
                "afk": false
              }
            }
          }
        "#));
        loop {
            if delta.elapsed().as_millis() == heartbeat_interval {
                println!("Sending heartbeat ...");
                client.send_message(&Message::text("{\"op\": 11}"));
                delta = std::time::Instant::now();
            }
        }
        Ok(())
    });

    loop {}

    Ok(())
}

async fn send_heartbeat(client: &mut websocket::client::sync::Client<TlsStream<TcpStream>>, heartbeat_state: &mut bool) {
    loop {
        thread::sleep(Duration::from_secs(15));
        client.send_message(&Message::text("{\"op\": 11}"));
    }
}