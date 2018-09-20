use actix::prelude::*;
use messages::{Envelope, RegisterRecipient, SendMessage, SendTyping};
use slack;
use APP_NAME;

pub struct SlackClient {
    sender: slack::Sender,
    recipients: Vec<Recipient<Envelope>>,
}

impl SlackClient {
    pub fn new(sender: slack::Sender) -> Self {
        Self {
            sender,
            recipients: Vec::new(),
        }
    }
}

impl Actor for SlackClient {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("slack_client started");
    }
}

impl Handler<RegisterRecipient<Envelope>> for SlackClient {
    type Result = ();
    fn handle(&mut self, msg: RegisterRecipient<Envelope>, _: &mut Self::Context) -> Self::Result {
        self.recipients.push(msg.into());
    }
}

impl Handler<Envelope> for SlackClient {
    type Result = <Envelope as actix::Message>::Result;
    fn handle(&mut self, msg: Envelope, _: &mut Self::Context) -> Self::Result {
        debug!("received envelope");
        if let Envelope::Event(evt) = msg {
            if match *evt {
                slack::Event::Hello => true,
                _ => false,
            } {
                info!("{} ready", APP_NAME);
            }
        } else {
            for recipient in &self.recipients {
                if let Err(e) = recipient.do_send(msg.clone()) {
                    error!("{}", e);
                }
            }
        }
    }
}

impl Handler<SendMessage> for SlackClient {
    type Result = <SendMessage as actix::Message>::Result;
    fn handle(&mut self, msg: SendMessage, _: &mut Self::Context) -> Self::Result {
        if let Err(e) = self.sender.send_message(msg.channel_id(), msg.message()) {
            error!("{}", e);
        }
    }
}

impl Handler<SendTyping> for SlackClient {
    type Result = <SendTyping as actix::Message>::Result;
    fn handle(&mut self, msg: SendTyping, _: &mut Self::Context) -> Self::Result {
        if let Err(e) = self.sender.send_typing(msg.channel_id()) {
            error!("{}", e);
        }
    }
}
