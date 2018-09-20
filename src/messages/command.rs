use slack_api;

#[derive(Clone, Debug, Message)]
pub struct Command {
    command: Option<String>,
    user: Option<String>,
    channel: Option<String>,
}

impl Command {
    pub fn new(command: Option<String>, user: Option<String>, channel: Option<String>) -> Self {
        Self {
            command: match command {
                Some(mut cmd) => Some(cmd.drain(config!(command_prefix).len()..).collect()),
                None => None,
            },
            user,
            channel,
        }
    }
    pub fn command(&self) -> &Option<String> {
        &self.command
    }
    pub fn user(&self) -> &Option<String> {
        &self.user
    }
    pub fn channel(&self) -> &Option<String> {
        &self.channel
    }
}

impl From<slack_api::MessageStandard> for Command {
    fn from(msg: slack_api::MessageStandard) -> Self {
        Command::new(msg.text, msg.user, msg.channel)
    }
}
