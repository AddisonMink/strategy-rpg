// fixed-size string type to use in constants

use std::fmt::{self, Display, Formatter};

const MAX_SHORT_STRING_SIZE: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortString {
    pub value: [u8; MAX_SHORT_STRING_SIZE],
    pub len: usize,
}

impl ShortString {
    pub const fn new(s: &str) -> Self {
        let bytes = s.as_bytes();
        let mut value = [0u8; MAX_SHORT_STRING_SIZE];
        let mut i = 0;
        while i < bytes.len() && i < MAX_SHORT_STRING_SIZE {
            value[i] = bytes[i];
            i += 1;
        }
        Self { value, len: i }
    }

    pub fn as_str(&self) -> &str {
        // SAFETY: value[0..len] is always valid UTF-8 if constructed from &str
        unsafe { std::str::from_utf8_unchecked(&self.value[..self.len]) }
    }
}

impl Display for ShortString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for ShortString {
    fn default() -> Self {
        Self {
            value: [0u8; MAX_SHORT_STRING_SIZE],
            len: 0,
        }
    }
}
