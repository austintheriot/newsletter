#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubscriberName(String);

impl AsRef<String> for SubscriberName {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

pub enum SubscriberEmailError {
    EmptyName,
}

impl SubscriberName {
    pub fn parse(
        subscriber_name: impl Into<String>,
    ) -> Result<SubscriberName, SubscriberEmailError> {
        let subscriber_name = subscriber_name.into();

        if subscriber_name.is_empty() {
            return Err(SubscriberEmailError::EmptyName);
        }

        Ok(SubscriberName(subscriber_name))
    }
}

impl TryFrom<String> for SubscriberName {
    type Error = SubscriberEmailError;

    fn try_from(subscriber_name: String) -> Result<Self, Self::Error> {
        Self::parse(subscriber_name)
    }
}
