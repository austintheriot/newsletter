use std::ops::Deref;

use thiserror::Error;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubscriberEmail(String);

#[derive(Error, Debug)]
pub enum SubscriberEmailError {
    #[error("email is invalid")]
    Invalid,
}

impl SubscriberEmail {
    pub fn parse(s: impl Into<String>) -> Result<SubscriberEmail, SubscriberEmailError> {
        let s: String = s.into();
        if validator::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(SubscriberEmailError::Invalid)
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};
    use fake::{faker::internet::en::SafeEmail, Fake};
    use quickcheck::{Arbitrary, Gen};

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl Arbitrary for ValidEmailFixture {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[test]
    fn invalid_emails_are_rejected() {
        let emails = [
            "",
            "@.com",
            "@",
            ".com",
            "@gmail.com",
            "austin",
            "austin.com",
            "austingmail.com",
            "austin@@gmail.com",
        ];
        for email in emails {
            let parsed_email = SubscriberEmail::parse(email);
            assert_err!(
                parsed_email,
                "Invalid email was not rejected by SubscriberEmail::parse: {email}"
            );
        }
    }

    // performs automated property-based testing
    #[quickcheck]
    fn valid_emails_are_parsed_successfully(email: ValidEmailFixture) {
        assert_ok!(SubscriberEmail::parse(email.0));
    }
}
