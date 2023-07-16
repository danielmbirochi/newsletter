use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid subscriber email", s))
        }
    }
}
    
impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use claims::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[derive(Debug,Clone)]
    struct ValidEmailFixture(String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[test]
    fn empty_string_is_rejected() {
        let empty = "".to_string();
        assert_err!(SubscriberEmail::parse(empty));
    }

    #[test]
    fn email_missing_symbol_is_rejected() {
        let email_missing_symbol = "somerandomstring".to_string();
        assert_err!(SubscriberEmail::parse(email_missing_symbol));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email_missing_subject = "@example.com".to_string();
        assert_err!(SubscriberEmail::parse(email_missing_subject));
    }

    #[test]
    fn email_with_subject_and_symbol_is_parsed_successfully() {
        let email_with_subject_and_symbol = "some@email.com".to_string();
        assert_ok!(SubscriberEmail::parse(email_with_subject_and_symbol));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        dbg!(&valid_email.0);
        SubscriberEmail::parse(valid_email.0).is_ok()
    }

}