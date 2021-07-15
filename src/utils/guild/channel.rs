use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TextChannel { // That's a correct TextChannel struct (I guess ...)
    id: String,
    channel_type: u16,
    guild_id: String,
    position: u32,
    permission_overwrites: Vec<Value>,
    name: String,
    topic: String,
    nsfw: bool,
    last_message_id: String,
    // bitrate: i32,
    // user_limit: i32,
    rate_limit_per_user: u16,
    // recipients: Vec<Value>,
    // icon: String,
    // owner_id: String,
    // application_id: String,
    parent_id: String,
    last_pin_timestamp: String,
    // rtc_region: String,
    // video_quality_mode: i32,
    // message_count: i32,
    // member_count: i32,
    // thread_metadata: Value,
    // member: Value,
    // default_auto_archive_duration: i32,
}
impl TextChannel {
    pub fn get_id(&self) -> &str {
        return &self.id;
    }
    pub fn get_name(&self) -> &str {
        return &self.name;
    }
    pub fn get_topic(&self) -> &str {
        return &self.topic;
    }
}