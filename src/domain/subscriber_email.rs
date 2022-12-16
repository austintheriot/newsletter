use std::{collections::HashSet, ops::Deref, vec};

use once_cell::sync::Lazy;
use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubscriberEmail(String);

#[derive(Error, Debug)]
pub enum SubscriberEmailError {
    #[error("email has no characters")]
    Empty,
    #[error("email exceeds the maximum length")]
    TooLong,
    #[error("email contains forbidden characters")]
    ForbiddenCharacters { forbidden_character: char },
}

pub static FORBIDDEN_CHARACTERS: Lazy<HashSet<char>> = Lazy::new(|| {
    vec!['\\', '<', '>', '/', '(', ')', '"', '{', '}']
        .into_iter()
        .collect()
});

impl SubscriberEmail {
    pub fn parse(s: impl Into<String>) -> Result<SubscriberEmail, SubscriberEmailError> {
        let s: String = s.into();

        if s.trim().is_empty() {
            return Err(SubscriberEmailError::Empty);
        } else if s.graphemes(true).count() > 255 {
            return Err(SubscriberEmailError::TooLong);
        }

        if let Some(forbidden_character) = s
            .chars()
            .into_iter()
            .find(|ch| FORBIDDEN_CHARACTERS.contains(ch))
        {
            return Err(SubscriberEmailError::ForbiddenCharacters {
                forbidden_character,
            });
        }

        Ok(SubscriberEmail(s))
    }

    pub fn inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for SubscriberEmail {
    type Error = SubscriberEmailError;

    fn try_from(subscriber_email: String) -> Result<Self, Self::Error> {
        Self::parse(subscriber_email)
    }
}

impl From<SubscriberEmail> for String {
    fn from(subscriber_email: SubscriberEmail) -> Self {
        subscriber_email.inner()
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for SubscriberEmail {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
