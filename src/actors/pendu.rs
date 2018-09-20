use actix::prelude::*;
use actors::SlackClient;
use messages::Envelope;

pub struct Pendu {
    slack_client: Addr<SlackClient>,
}

impl Pendu {
    pub fn new(slack_client: Addr<SlackClient>) -> Self {
        Self { slack_client }
    }
}

impl Actor for Pendu {
    type Context = Context<Self>;
}

impl Handler<Envelope> for Pendu {
    type Result = ();
    fn handle(&mut self, msg: Envelope, _: &mut Self::Context) -> Self::Result {
        debug!("received event");
        debug!("{:?}", msg);
    }
}
