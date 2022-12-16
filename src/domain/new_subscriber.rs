use thiserror::Error;

use crate::{SubscriberEmailParseError, SubscriberNameError};

use super::{SubscriberEmail, SubscriberNameParseError};

#[derive(Error, Debug)]
pub enum NewSubscriberParseError {
    #[error("name is invalid: {0:?}")]
    NameError(#[from] SubscriberNameError),
    #[error("email is invalid: {0:?}")]
    EmailError(#[from] SubscriberEmailParseError),
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NewSubscriber {
    pub name: SubscriberNameParseError,
    pub email: SubscriberEmail,
}

impl NewSubscriber {
    pub fn parse(
        name: impl Into<String>,
        email: impl Into<String>,
    ) -> Result<Self, NewSubscriberParseError> {
        let name = SubscriberNameParseError::parse(name)?;
        let email = SubscriberEmail::parse(email)?;
        Ok(Self { name, email })
    }
}

impl AsRef<SubscriberNameParseError> for NewSubscriber {
    fn as_ref(&self) -> &SubscriberNameParseError {
        &self.name
    }
}

impl AsRef<SubscriberEmail> for NewSubscriber {
    fn as_ref(&self) -> &SubscriberEmail {
        &self.email
    }
}
