use crate::{CharCoord, EndOfLine};
use derive_more::{Display, From, TryInto};
use getset::{CopyGetters, Getters};
use std::fmt::{self, Debug, Display, Formatter};

/// Information of a single character.
#[derive(Clone, Copy, CopyGetters, Getters)]
pub struct CharCell<Char> {
    /// Character coordinate.
    #[getset(get_copy = "pub")]
    pub(crate) coord: CharCoord,
    /// Byte offset from the start of the line.
    #[getset(get_copy = "pub")]
    pub(crate) offset_from_ln_start: usize,
    /// Byte offset from the start of the document.
    #[getset(get_copy = "pub")]
    pub(crate) offset_from_doc_start: usize,
    /// Content of the character.
    #[getset(get = "pub")]
    pub(crate) value: Char,
}

impl<Char> CharCell<Char> {
    /// Map a `CharCell<Char>` to a `CharCell<Return>` by applying a function.
    pub(crate) fn map<Return, Function>(self, function: Function) -> CharCell<Return>
    where
        Function: FnOnce(Char) -> Return,
    {
        let CharCell {
            coord,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        } = self;
        let value = function(value);
        CharCell {
            coord,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        }
    }
}

impl<Char: Display> Display for CharCell<Char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl<Char: Debug> Debug for CharCell<Char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let CharCell { value, coord, .. } = self;
        write!(f, "CharCell at {coord} of {value:?}")
    }
}

/// Either a non-EOL character or and EOL sequence.
#[derive(Debug, Display, Clone, Copy, From, TryInto)]
pub enum CharOrEol {
    /// Non-EOL character.
    Char(char),
    /// End of line sequence.
    EndOfLine(EndOfLine),
}

impl TryFrom<CharCell<CharOrEol>> for CharCell<char> {
    type Error = &'static str;

    fn try_from(char_cell: CharCell<CharOrEol>) -> Result<Self, Self::Error> {
        let CharCell {
            coord,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        } = char_cell;
        value.try_into().map(|value| CharCell {
            coord,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        })
    }
}

impl TryFrom<CharCell<CharOrEol>> for CharCell<EndOfLine> {
    type Error = &'static str;

    fn try_from(char_cell: CharCell<CharOrEol>) -> Result<Self, Self::Error> {
        let CharCell {
            coord,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        } = char_cell;
        value.try_into().map(|value| CharCell {
            coord,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        })
    }
}
