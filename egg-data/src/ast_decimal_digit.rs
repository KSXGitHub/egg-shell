use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, Error, Into, IntoIterator};
use pipe_trait::Pipe;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Value between 0 and 9.
#[derive(
    Debug,
    Display,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    AsRef,
    Deref,
    Into,
    Serialize,
    Deserialize,
)]
pub struct AstDecimalDigit(u8);

impl AstDecimalDigit {
    /// Create a digit from a character.
    pub const fn from_char(char: char) -> Option<Self> {
        if !char.is_ascii_digit() {
            return None;
        }
        let value = (char as u8) - b'0';
        Some(AstDecimalDigit(value))
    }

    /// Convert the digit to [`u8`].
    pub const fn value(self) -> u8 {
        self.0
    }
}

/// List of [digits](AstDecimalDigit).
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    AsMut,
    AsRef,
    Deref,
    DerefMut,
    Into,
    IntoIterator,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "String", into = "String")]
pub struct AstDecimalDigitList(Vec<AstDecimalDigit>);

/// Error for converting a string to a [digit list](AstDecimalDigitList).
#[derive(Debug, Display, Clone, PartialEq, Eq, Error)]
#[display(fmt = "Failed to convert the character {char:?} at {index} to a digit")]
pub struct AstDecimalDigitListFromStrError {
    /// Index of the character that cause the error.
    pub index: usize,
    /// Character that cause the error.
    pub char: char,
    /// Previous digits.
    pub prev: Vec<AstDecimalDigit>,
}

impl FromStr for AstDecimalDigitList {
    type Err = AstDecimalDigitListFromStrError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut list = Vec::with_capacity(value.len());
        for (index, char) in value.char_indices() {
            match AstDecimalDigit::from_char(char) {
                Some(item) => list.push(item),
                None => {
                    return Err(AstDecimalDigitListFromStrError {
                        index,
                        char,
                        prev: list,
                    })
                }
            }
        }
        list.pipe(AstDecimalDigitList).pipe(Ok)
    }
}

impl TryFrom<String> for AstDecimalDigitList {
    type Error = AstDecimalDigitListFromStrError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Display for AstDecimalDigitList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for digit in self.iter() {
            write!(f, "{digit}")?
        }
        Ok(())
    }
}

impl From<AstDecimalDigitList> for String {
    fn from(value: AstDecimalDigitList) -> Self {
        value.to_string()
    }
}
