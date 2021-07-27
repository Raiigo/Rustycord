use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SessionStartLimit {
    total: u32,
    remaining: u32,
    reset_after: u32,
    max_concurrency: u32,
}

#[derive(Deserialize, Debug)]
pub struct GatewayInfos {
    url: String,
    shards: u8,
    session_start_limit: SessionStartLimit,
}
impl GatewayInfos {
    pub fn get_url(&self) -> &str {
        return &self.url;
    }
}