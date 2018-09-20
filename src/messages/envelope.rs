use messages::Command;
use slack;

#[derive(Clone, Debug, Message)]
pub enum Envelope {
    Command(Box<Command>),
    Message(Box<slack::Message>),
    Event(Box<slack::Event>),
}

impl From<slack::Event> for Envelope {
    fn from(evt: slack::Event) -> Self {
        match evt {
            slack::Event::Message(msg) => match *msg.clone() {
                slack::Message::Standard(m) => match m.text.clone() {
                    Some(text) => {
                        if text.starts_with(config!(command_prefix)) {
                            Envelope::Command(Box::new(m.into()))
                        } else {
                            Envelope::Message(msg)
                        }
                    }
                    _ => Envelope::Message(msg),
                },
                _ => Envelope::Message(msg),
            },
            _ => Envelope::Event(Box::new(evt)),
        }
    }
}
