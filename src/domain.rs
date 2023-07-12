use unicode_segmentation::UnicodeSegmentation;


pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> SubscriberName {
        let is_empty_or_whitespace = s.trim().is_empty();

        // A grapheme is defined by the Unicode standard as a "user-perceived" character:
        // https://unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries
        let is_too_long = s.graphemes(true).count() > 256;
    
        let forbidden_characters = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', '{', '}', '(', ')', '[', ']', '&', '$', '#', '@', '%', '^', '!', '~', '`', '+', '=', ';', ','];
        let contains_forbidden_characters = s
            .chars()
            .any(|c| forbidden_characters.contains(&c));
    
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            panic!("{} invalid subscriber name", s)
        } else {
            Self(s)
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}