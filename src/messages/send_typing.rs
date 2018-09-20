#[derive(Message)]
pub struct SendTyping {
    channel_id: String,
}

impl SendTyping {
    pub fn new(channel_id: &str) -> Self {
        Self {
            channel_id: channel_id.to_string(),
        }
    }
    pub fn channel_id(&self) -> &str {
        &self.channel_id
    }
}
