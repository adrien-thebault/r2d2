use actix::prelude::*;

#[derive(Message)]
pub struct RegisterRecipient<M>
where
    M: Message + Send,
    M::Result: Send,
{
    recipient: Recipient<M>,
}

impl<M> RegisterRecipient<M>
where
    M: Message + Send,
    M::Result: Send,
{
    pub fn new(recipient: Recipient<M>) -> Self {
        Self { recipient }
    }
}

impl<M> Into<Recipient<M>> for RegisterRecipient<M>
where
    M: Message + Send,
    M::Result: Send,
{
    fn into(self) -> Recipient<M> {
        self.recipient
    }
}
