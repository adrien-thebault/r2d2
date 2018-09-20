#[derive(Message)]
pub struct SendMessage {
    message: String,
    channel_id: String,
}

impl SendMessage {
    pub fn new(channel_id: &str, message: &str) -> Self {
        Self {
            channel_id: channel_id.to_string(),
            message: message.to_string(),
        }
    }
    pub fn channel_id(&self) -> &str {
        &self.channel_id
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}
