use reqwest::*;
use tokio::*;
use serde::{Serialize, Deserialize};

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

    let request = Client::new()
        .request(Method::GET, "https://discord.com/api/v9/gateway/bot")
        .header("Authorization", "Bot TOKEN")
        .send().await?.json::<GatewayInfos>().await?;

    println!("{}", request.get_url());

    Ok(())
}
