use serde_json::Value;

pub struct TextChannel {
    id: String,
    guild_id: String,
    name: String,
    channel_type: u16,
    position: u32,
    permission_overwrites: Vec<Value>,
    rate_limit_per_user: u16,
    nsfw: bool,
    topic: String,
    last_message_id: String,
    parent_id: String,
    default_auto_archive_duration: u32,
}