use std::{collections::HashSet, ops::Deref, vec};

use once_cell::sync::Lazy;
use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubscriberNameParseError(String);

#[derive(Error, Debug)]
pub enum SubscriberNameError {
    #[error("name has no characters")]
    Empty,
    #[error("name exceeds the maximum length")]
    TooLong,
    #[error("name contains forbidden characters")]
    ForbiddenCharacters { forbidden_character: char },
}

pub static FORBIDDEN_CHARACTERS: Lazy<HashSet<char>> = Lazy::new(|| {
    vec!['\\', '<', '>', '/', '(', ')', '"', '{', '}']
        .into_iter()
        .collect()
});

impl SubscriberNameParseError {
    pub fn parse(s: impl Into<String>) -> Result<SubscriberNameParseError, SubscriberNameError> {
        let s: String = s.into();

        if s.trim().is_empty() {
            return Err(SubscriberNameError::Empty);
        } else if s.graphemes(true).count() > 256 {
            return Err(SubscriberNameError::TooLong);
        }

        if let Some(forbidden_character) = s
            .chars()
            .into_iter()
            .find(|ch| FORBIDDEN_CHARACTERS.contains(ch))
        {
            return Err(SubscriberNameError::ForbiddenCharacters {
                forbidden_character,
            });
        }

        Ok(SubscriberNameParseError(s))
    }

    pub fn inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for SubscriberNameParseError {
    type Error = SubscriberNameError;

    fn try_from(subscriber_email: String) -> Result<Self, Self::Error> {
        Self::parse(subscriber_email)
    }
}

impl From<SubscriberNameParseError> for String {
    fn from(subscriber_email: SubscriberNameParseError) -> Self {
        subscriber_email.inner()
    }
}

impl AsRef<str> for SubscriberNameParseError {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for SubscriberNameParseError {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_name_is_valid() {
        let name = SubscriberNameParseError::parse("a̐".repeat(256));
        assert_ok!(name);
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = SubscriberNameParseError::parse("a".repeat(257));
        assert_err!(name);
    }

    #[test]
    fn a_name_consisting_of_whitespace_is_rejected() {
        let name = SubscriberNameParseError::parse(" ".repeat(10));
        assert_err!(name);
    }

    #[test]
    fn an_empty_name_is_rejected() {
        let name = SubscriberNameParseError::parse("");
        assert_err!(name);
    }

    #[test]
    fn a_name_containing_invalid_characters_is_rejected() {
        for ch in FORBIDDEN_CHARACTERS.iter() {
            let name = SubscriberNameParseError::parse(*ch);
            assert_err!(
                name,
                "SubscribeName failed to reject invalid character: {ch}"
            );
        }
    }

    #[test]
    fn a_valid_name_is_accepted() {
        for name in ["Austin Theriot", "ナルト", "אברהם"] {
            let name = SubscriberNameParseError::parse(name);
            assert_ok!(name, "SubscribeName failed to parse valid name");
        }
    }
}
