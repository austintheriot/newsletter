use std::{collections::HashSet, vec};

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubscriberEmail(String);

impl AsRef<String> for SubscriberEmail {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

pub enum SubscriberEmailError {
    Empty,
    TooLong,
    ForbiddenCharacters { forbidden_character: char },
}

impl SubscriberEmail {
    pub fn parse(s: impl Into<String>) -> Result<SubscriberEmail, SubscriberEmailError> {
        let s: String = s.into();

        if s.trim().is_empty() {
            return Err(SubscriberEmailError::Empty);
        } else if s.graphemes(true).count() > 255 {
            return Err(SubscriberEmailError::TooLong);
        }

        let forbidden_characters: HashSet<char> =
            vec!['\\', '<', '>', '/', '(', ')', '"', '{', '}']
                .into_iter()
                .collect();
        if let Some(forbidden_character) = s
            .chars()
            .into_iter()
            .find(|ch| forbidden_characters.contains(&ch))
        {
            return Err(SubscriberEmailError::ForbiddenCharacters {
                forbidden_character,
            });
        }

        Ok(SubscriberEmail(s.to_string()))
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
