use actix::prelude::*;
use actors::SlackClient;
use messages::Envelope;

pub struct Quiz {
    slack_client: Addr<SlackClient>,
}

impl Quiz {
    pub fn new(slack_client: Addr<SlackClient>) -> Self {
        Self { slack_client }
    }
}

impl Actor for Quiz {
    type Context = Context<Self>;
}

impl Handler<Envelope> for Quiz {
    type Result = ();
    fn handle(&mut self, msg: Envelope, _: &mut Self::Context) -> Self::Result {
        debug!("received event");
        debug!("{:?}", msg);
    }
}
