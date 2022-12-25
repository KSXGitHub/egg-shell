use crate::CharCoord;
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
