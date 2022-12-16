use super::{SubscriberEmail, SubscriberName};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}

impl AsRef<SubscriberName> for NewSubscriber {
    fn as_ref(&self) -> &SubscriberName {
        &self.name
    }
}

impl AsRef<SubscriberEmail> for NewSubscriber {
    fn as_ref(&self) -> &SubscriberEmail {
        &self.email
    }
}
