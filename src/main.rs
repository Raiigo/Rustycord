pub mod utils;
pub mod core;

use std::{net::TcpStream, thread, time::Duration};
use reqwest::*;
use tokio::*;
use serde::{Serialize, Deserialize};
use websocket::{ClientBuilder, Message, header::Authorization};
use native_tls::*;
use io;
use serde_json::Value;
use crate::utils::guild::channel::*;
use crate::core::bot::Bot;

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

#[derive(Deserialize)]
struct GuildMember {
    roles: Vec<String>,
    premium_since: String,
    pending: bool,
    nick: String,
    mute: bool,
    joined_at: String,
    is_pending: bool,
    hoisted_role: String,
    deaf: bool,
    avatar: String,
}

#[derive(Deserialize)]
struct User {
    username: String,
    public_flags: i32,
    id: String,
    discriminator: String,
    avatar: String,
}

#[derive(Deserialize)]
struct DiscordMessage {
    message_type: u16,
    tts: bool, // Is that message a TTS message
    timestanp: String, // Time
    referenced_message: String,
    pinned: bool,
    nonce: String,
    mentions: Vec<String>,
    mention_roles: Vec<String>,
    mention_everyone: bool,
    member: GuildMember,
    id: String,
    flags: i32,
    embeds: Vec<String>,
    edited_timestamp: String,
    content: String,
    components: Vec<String>,
    channel_id: String,
    author: User,
    attachments: Vec<String>,
    guild_id: String,
}

#[derive(Deserialize)]
struct OpCode0 { // Event dispatched
    t: String, // Event name (MESSAGE_CREATE)
    s: i32,
    d: DiscordMessage, // Message
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {

    
    

    // thread::spawn(move || {
    //     let bot = Bot::new("token_artifact", Box::new(|| {
    //         true
    //     }));
    //     let gateway = bot.get_gateway_infos();
    //     println!("Gateway info from bot struct : ");
    //     dbg!(gateway);
    // });
    

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
        .connect_secure(None).unwrap();

    match client.set_nonblocking(true) {
        Ok(_) => println!("Nonblocking"),
        Err(_error) => panic!(),
    }

    let mut text_message = match client.recv_message().unwrap() {
        websocket::OwnedMessage::Text(text) => text,
        websocket::OwnedMessage::Binary(_) => String::new(),
        websocket::OwnedMessage::Close(_) => String::new(),
        websocket::OwnedMessage::Ping(_) => String::new(),
        websocket::OwnedMessage::Pong(_) => String::new(),
    };

    text_message = text_message.replace("\"t\":null,\"s\":null,", "");
    text_message.replace_range(40.., "}}");
    text_message = text_message.replace(r#"{"op":10,"d":{"heartbeat_interval":"#, "").replace("}}", "");
    
    println!("{}", text_message);

    let heartbeat_interval: u128 = text_message.parse::<u128>().unwrap();

    thread::spawn(move || -> reqwest::Result<()> {
        let mut delta = std::time::Instant::now();
        let _response = client.send_message(&Message::text(r#"
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
                client.send_message(&Message::text(r#"{"op": 1,"d": null}"#)).map_or_else(|e|{
                    dbg!(e);
                    println!("Error while sending heartbeat");
                }, |_| {
                    println!("Heartbeat sent !");
                });
                delta = std::time::Instant::now();
            }
            for message in client.recv_message() {
                let message_text = match message {
                    websocket::OwnedMessage::Text(text) => text,
                    websocket::OwnedMessage::Binary(_) => String::new(),
                    websocket::OwnedMessage::Close(_) => String::new(),
                    websocket::OwnedMessage::Ping(_) => String::new(),
                    websocket::OwnedMessage::Pong(_) => String::new(),
                };
                println!("{}", message_text);
                let v: Value = serde_json::from_str(&message_text).unwrap(); // Représente le message OP 0
                println!("{}", v["d"]["content"]);
                let channel_id: String;
                channel_id = match &v["d"]["channel_id"] {
                    Value::Null => {
                        println!("Channel ID doesn't exist");
                        String::new()
                    },
                    Value::Bool(_) => {
                        println!("Channel ID doesn't exist");
                        String::new()
                    },
                    Value::Number(_) => {
                        println!("Channel ID doesn't exist");
                        String::new()
                    },
                    Value::String(id) => String::from(id),
                    Value::Array(_) => {
                        println!("Channel ID doesn't exist");
                        String::new()
                    },
                    Value::Object(_) => {
                        println!("Channel ID doesn't exist");
                        String::new()
                    },
                };
                let content: String = match &v["d"]["content"] {
                    Value::Null => String::new(),
                    Value::Bool(_) => String::new(),
                    Value::Number(_) => String::new(),
                    Value::String(text) => String::from(text),
                    Value::Array(_) => String::new(),
                    Value::Object(_) => String::new(),
                };
                println!("{}", content);
                if content.replace("\"", "").eq("!get_channels") {
                    let guild_id = match &v["d"]["guild_id"] {
                        Value::Null => String::from("Guild ID not found"),
                        Value::Bool(_) => String::from("Guild ID not found"),
                        Value::Number(_) => String::from("Guild ID not found"),
                        Value::String(text) => String::from(text),
                        Value::Array(_) => String::from("Guild ID not found"),
                        Value::Object(_) => String::from("Guild ID not found"),
                    };
                    let url = format!("https://discord.com/api/v9/guilds/{}/channels", guild_id);
                    println!("{}", url);
                    let res = reqwest::blocking::Client::new().get(url)
                        .header("Authorization", "Bot token_artifact")
                        .send().unwrap();
                    let mut res_text = res.text().unwrap();
                    if res_text.len() < 2000 {
                        println!("<2000");
                        let body = format!("{{
                            \"content\": \"{}\",
                            \"tts\": false
                        }}", res_text);
                        let res_post = reqwest::blocking::Client::new().post(format!("https://discord.com/api/v9/channels/{}/messages", channel_id))
                            .header("Authorization", "Bot token_artifact")
                            .header("Content-type", "application/json")
                            .body(body).send().unwrap();
                    } else {
                        println!(">2000");
                        println!("{}", res_text);
                        let channels: Value = serde_json::from_str(&res_text).unwrap();
                        match channels {
                            Value::Null => println!(),
                            Value::Bool(_) => println!(),
                            Value::Number(_) => println!(),
                            Value::String(_) => println!(),
                            Value::Array(list) => {
                                let number = list.len();
                                let body = format!("{{
                                    \"content\": \"Il y a : {} channels\",
                                    \"tts\": false
                                }}", number);
                                let res_post = reqwest::blocking::Client::new().post(format!("https://discord.com/api/v9/channels/{}/messages", channel_id))
                                    .header("Authorization", "Bot token_artifact")
                                    .header("Content-type", "application/json")
                                    .body(body).send().unwrap();
                                dbg!(res_post);
                                let mut channel_list: String = String::new();
                                println!("----------------------------------------------------");
                                println!("{}", &list[4].to_string());
                                println!("----------------------------------------------------");
                                let voc1_json: &String = &list[4].to_string().replace("type", "channel_type");
                                let voc1: TextChannel = serde_json::from_str(voc1_json).unwrap();
                                dbg!(voc1);
                                for channel in list {
                                    println!("{}", channel);
                                    let name = match &channel["name"] {
                                        Value::Null => String::from("Error while getting the name"),
                                        Value::Bool(_) => String::from("Error while getting the name"),
                                        Value::Number(_) => String::from("Error while getting the name"),
                                        Value::String(text) => String::from(text),
                                        Value::Array(_) => String::from("Error while getting the name"),
                                        Value::Object(_) => String::from("Error while getting the name"),
                                    };
                                    channel_list.push_str(&name);
                                    channel_list.push_str(",");
                                }
                                let body = format!("{{
                                    \"content\": \"{}\",
                                    \"tts\": false
                                }}", channel_list.replace("\"", r#"\""#));
                                let res_post = reqwest::blocking::Client::new().post(format!("https://discord.com/api/v9/channels/{}/messages", channel_id))
                                    .header("Authorization", "Bot token_artifact")
                                    .header("Content-type", "application/json")
                                    .body(body).send().unwrap();
                                dbg!(res_post);
                            },
                            Value::Object(_) => println!(),
                        }
                        res_text.truncate(1900);
                        let body = format!("{{
                            \"content\": \"```{}```\",
                            \"tts\": false
                        }}", res_text.replace("\"", r#"\""#));
                        println!("{}", body);
                        // let res_post = reqwest::blocking::Client::new().post(format!("https://discord.com/api/v9/channels/{}/messages", channel_id))
                        //     .header("Authorization", "Bot token_artifact")
                        //     .header("Content-type", "application/json")
                        //     .body(body).send().unwrap();
                        // dbg!(res_post);
                    }
                }

                
                if content.replace("\"", "").starts_with("!kda") {
                    let player_name = content.replace("\"", "").replace("!kda", "").replace(" ", "");
                    println!("KDA command detected !");
                    let kda_res = reqwest::blocking::Client::new().get(format!("http://127.0.0.1/kda/{}", player_name))
                        .send().unwrap().text().unwrap();
                    println!("{}", kda_res);
                    let mut kda_list: Vec<String> = kda_res.split("\n").map(|e| {
                         e.to_owned()
                    }).collect();
                    kda_list.remove(10);
                    let list_len = 10;
                    let mut kill_sum: i32 = 0;
                    let mut death_sum: i32 = 0;
                    let mut assist_sum: i32 = 0;
                    for kda in kda_list {
                        let kda_clean = kda.replace("\n", "");
                        println!("{}", kda_clean);
                        // println!("{}", kda.split('/').nth(0).unwrap());
                        // println!("{}", kda.split('/').nth(1).unwrap());
                        // println!("{}", kda.split('/').nth(2).unwrap());
                        let comp_list: Vec<i32> = kda_clean.split('/').map(|e| {
                            println!("{}", e);
                            e.trim().parse::<i32>().unwrap()
                        }).collect();
                        // for c in comp_list {
                        //     println!("{}", c);
                        // }
                        let kill: i32 = comp_list[0];
                        kill_sum = kill_sum + kill;
                        let death: i32 = comp_list[1];
                        death_sum = death_sum + death;
                        let assist: i32 = comp_list[2];
                        assist_sum = assist_sum + assist;
                    }
                    let kill_mean: f64 = f64::from(kill_sum) / f64::from(list_len);
                    let death_mean: f64 = f64::from(death_sum) / f64::from(list_len);
                    let assist_mean: f64 = f64::from(assist_sum) / f64::from(list_len);
                    let kda_mean = format!("{}/{}/{}", kill_mean, death_mean, assist_mean);
                    let res = reqwest::blocking::Client::new().post(format!("https://discord.com/api/v9/channels/{}/messages", channel_id))
                        .header("Authorization", "Bot token_artifact")
                        .header("Content-type", "application/json")
                        .body(format!("{{
                            \"content\": \"{}\",
                            \"tts\": false
                        }}", kda_res.replace("\n", "\\n"))).send().unwrap();
                    dbg!(res);
                    let res = reqwest::blocking::Client::new().post(format!("https://discord.com/api/v9/channels/{}/messages", channel_id))
                        .header("Authorization", "Bot token_artifact")
                        .header("Content-type", "application/json")
                        .body(format!("{{
                            \"content\": \"Moyenne : {}\",
                            \"tts\": false
                        }}", kda_mean)).send().unwrap();
                    dbg!(res);
                }

                if content.replace("\"", "").eq("!test") {
                    println!("Command !test found !");
                    // channel_id = match &v["d"]["channel_id"] {
                    //     Value::Null => {
                    //         println!("Channel ID doesn't exist");
                    //         String::new()
                    //     },
                    //     Value::Bool(_) => {
                    //         println!("Channel ID doesn't exist");
                    //         String::new()
                    //     },
                    //     Value::Number(_) => {
                    //         println!("Channel ID doesn't exist");
                    //         String::new()
                    //     },
                    //     Value::String(id) => String::from(id),
                    //     Value::Array(_) => {
                    //         println!("Channel ID doesn't exist");
                    //         String::new()
                    //     },
                    //     Value::Object(_) => {
                    //         println!("Channel ID doesn't exist");
                    //         String::new()
                    //     },
                    // };
                    let url: String = format!("https://discord.com/api/v9/channels/{}/messages", channel_id);
                    println!("{}", url);
                    if v["d"]["author"]["id"].eq("320522831362523137") {
                        let name: String = match &v["d"]["member"]["nick"] {
                            Value::Null => String::from("GUAPO"),
                            Value::Bool(_) => String::from("GUAPO"),
                            Value::Number(_) => String::from("GUAPO"),
                            Value::String(text) => String::from(text),
                            Value::Array(_) => String::from("GUAPO"),
                            Value::Object(_) => String::from("GUAPO"),
                        };
                        let body = format!("{{
                            \"content\": \"TG {}\",
                            \"tts\": false
                        }}", name);
                        let res = reqwest::blocking::Client::new().post(format!("https://discord.com/api/v9/channels/{}/messages", channel_id))
                        .header("Authorization", "Bot token_artifact")
                        .header("Content-type", "application/json")
                        .body(body).send().unwrap();
                        dbg!(res);
                    } else if v["d"]["author"]["id"].eq("257231064362254336") {
                        let res = reqwest::blocking::Client::new().post(format!("https://discord.com/api/v9/channels/{}/messages", channel_id))
                        .header("Authorization", "Bot token_artifact")
                        .header("Content-type", "application/json")
                        .body(r#"{
                            "content": "Tu veux jouer à R6 ?",
                            "tts": false
                        }"#).send().unwrap();
                        dbg!(res);
                    } else if v["d"]["author"]["id"].eq("285213193041608704") {
                        let res = reqwest::blocking::Client::new().post(format!("https://discord.com/api/v9/channels/{}/messages", channel_id))
                        .header("Authorization", "Bot token_artifact")
                        .header("Content-type", "application/json")
                        .body(r#"{
                            "content": "Tu dois 30€ à mon créateur petit batard",
                            "tts": false
                        }"#).send().unwrap();
                        dbg!(res);
                    } else {
                        let res = reqwest::blocking::Client::new().post(format!("https://discord.com/api/v9/channels/{}/messages", channel_id))
                        .header("Authorization", "Bot token_artifact")
                        .header("Content-type", "application/json")
                        .body(r#"{
                            "content": "Bonjour Homme respectable",
                            "tts": false
                        }"#).send().unwrap();
                        dbg!(res);
                    }

                }
            }
        }
    });

    loop {}

}