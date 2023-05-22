use derive_more::Error;
use std::fmt::{self, Display, Formatter};

/// Token represents an invalid (unparsable) character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub struct InvalidToken(#[error(not(source))] pub char);

impl Display for InvalidToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let InvalidToken(char) = self;
        let code = *char as u32;
        write!(f, "Unexpected token {char:?} (U+{code:04X})")
    }
}
