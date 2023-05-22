use crate::token::{whitespace::WhitespaceTokenParseError, WhitespaceToken};
use derive_more::{Display, Error, From, Into};
use pipe_trait::Pipe;
use std::fmt::{self, Display, Formatter};

/// Character for indentation. Either a space or a tab.
///
/// Use [`TryFrom<char>`] to create an `IndentChar`.
///
/// Use [`Into<char>`] to extract the original character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, From, Into)]
pub struct IndentChar(pub WhitespaceToken);

impl IndentChar {
    /// Get 3-letter abbreviation.
    pub(crate) const fn abbr(self) -> &'static str {
        let IndentChar(ws) = self;
        match ws {
            WhitespaceToken::Space => "SPC",
            WhitespaceToken::Tab => "TAB",
        }
    }
}

impl From<IndentChar> for char {
    fn from(input: IndentChar) -> Self {
        input.pipe(WhitespaceToken::from).into()
    }
}

/// Error when failing to [convert](TryFrom) a `char` to an [`IndentChar`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
#[display(fmt = "Cannot convert {_0:?} to an indentation")]
pub struct IndentCharParseError(#[error(not(source))] char);

impl IndentCharParseError {
    /// Get the original input.
    pub const fn input(self) -> char {
        self.0
    }
}

impl TryFrom<char> for IndentChar {
    type Error = IndentCharParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        value
            .pipe(WhitespaceToken::try_from)
            .map(IndentChar::from)
            .map_err(WhitespaceTokenParseError::input)
            .map_err(IndentCharParseError)
    }
}

impl Display for IndentChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        char::from(*self).fmt(f)
    }
}
