

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if s.is_empty() {
            Err("email cannot be empty".into())
        } else {
            Ok(Self(s))
        }
    }
}
    
impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}