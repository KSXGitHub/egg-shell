use derive_more::{Display, Error};

/// Quote type of [`StringToken`](super::StringToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quote {
    /// Single quotes (`'`) were used to wrap the string content.
    Single,
    /// Double quotes (`"`) were used to wrap the string content.
    Double,
}

impl Quote {
    /// Infer a quote from a character.
    pub const fn from_char(char: char) -> Option<Self> {
        match char {
            '\'' => Some(Quote::Single),
            '"' => Some(Quote::Double),
            _ => None,
        }
    }

    /// Infer a char from the quote type.
    pub const fn to_char(&self) -> char {
        match self {
            Quote::Single => '\'',
            Quote::Double => '"',
        }
    }
}

/// Error when failing to [convert](TryFrom) a `char` to an [`Quote`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
#[display(fmt = "Cannot convert {_0:?} to a quote type")]
pub struct QuoteParseError(#[error(not(source))] char);

impl TryFrom<char> for Quote {
    type Error = QuoteParseError;
    fn try_from(char: char) -> Result<Self, Self::Error> {
        Quote::from_char(char).ok_or(QuoteParseError(char))
    }
}

impl From<Quote> for char {
    fn from(quote: Quote) -> Self {
        quote.to_char()
    }
}
