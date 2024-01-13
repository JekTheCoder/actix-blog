pub struct HashedPassword(String);

impl HashedPassword {
    pub fn new_unchecked(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
