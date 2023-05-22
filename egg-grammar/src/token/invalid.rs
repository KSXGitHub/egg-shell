use derive_more::Error;
use split_first_char::split_first_char;
use std::fmt::{self, Display, Formatter};

/// Token represents an invalid (unparsable) character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub struct InvalidToken(#[error(not(source))] pub char);

impl InvalidToken {
    /// Create an [`InvalidToken`] from the first character of a string.
    pub fn parse(input: &str) -> Option<(Self, &'_ str)> {
        let (char, rest) = split_first_char(input)?;
        let token = InvalidToken(char);
        Some((token, rest))
    }
}

impl Display for InvalidToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let InvalidToken(char) = self;
        let code = *char as u32;
        write!(f, "Unexpected token {char:?} (U+{code:04X})")
    }
}
