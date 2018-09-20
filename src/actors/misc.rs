use actix::prelude::*;
use actors::SlackClient;
use messages::Envelope;

pub struct Misc {
    slack_client: Addr<SlackClient>,
}

impl Misc {
    pub fn new(slack_client: Addr<SlackClient>) -> Self {
        Self { slack_client }
    }
}

impl Actor for Misc {
    type Context = Context<Self>;
}

impl Handler<Envelope> for Misc {
    type Result = ();
    fn handle(&mut self, msg: Envelope, _: &mut Self::Context) -> Self::Result {
        debug!("received event");
        debug!("{:?}", msg);
    }
}
