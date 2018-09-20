use actix::prelude::*;
use actors::SlackClient;
use messages::Envelope;

pub struct Admin {
    slack_client: Addr<SlackClient>,
}

impl Admin {
    pub fn new(slack_client: Addr<SlackClient>) -> Self {
        Self { slack_client }
    }
}

impl Actor for Admin {
    type Context = Context<Self>;
}

impl Handler<Envelope> for Admin {
    type Result = ();
    fn handle(&mut self, msg: Envelope, _: &mut Self::Context) -> Self::Result {
        debug!("received event");
        debug!("{:?}", msg);
    }
}
