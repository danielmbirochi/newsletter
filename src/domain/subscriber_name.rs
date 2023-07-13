use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        // A grapheme is defined by the Unicode standard as a "user-perceived" character:
        // https://unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries
        let is_too_long = s.graphemes(true).count() > 256;
    
        let forbidden_characters = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', '{', '}', '(', ')', '[', ']', '&', '$', '#', '@', '%', '^', '!', '~', '`', '+', '=', ';', ','];
        let contains_forbidden_characters = s
            .chars()
            .any(|c| forbidden_characters.contains(&c));
    
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} invalid subscriber name", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn empty_string_is_rejected() {
        let empty = "".to_string();
        assert_err!(SubscriberName::parse(empty));
    }

    #[test]
    fn whitespace_only_string_is_rejected() {
        let whitespace = " ".to_string();
        assert_err!(SubscriberName::parse(whitespace));
    }

    #[test]
    fn string_with_more_than_256_graphemes_is_rejected() {
        let long_string = "a".repeat(257);
        assert_err!(SubscriberName::parse(long_string));
    }

    #[test]
    fn string_with_forbidden_characters_is_rejected() {
        let forbidden_characters = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', '{', '}', '(', ')', '[', ']', '&', '$', '#', '@', '%', '^', '!', '~', '`', '+', '=', ';', ','];
        for c in forbidden_characters.iter()  {
            let string_with_forbidden_character = format!("a{}", c);
            assert_err!(SubscriberName::parse(string_with_forbidden_character));
        }
    }

    #[test]
    fn valid_subscriber_name_is_parsed_successfully() {
        let valid_subscriber_name = "valid_subscriber_name".to_string();
        assert_ok!(SubscriberName::parse(valid_subscriber_name));
    }

    #[test]
    fn valid_subscriber_name_with_256_graphemes_is_parsed_successfully() {
        let valid_subscriber_name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(valid_subscriber_name));
    }

}