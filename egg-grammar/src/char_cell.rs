use crate::CharCoord;
use getset::{CopyGetters, Getters};
use std::fmt::{self, Debug, Display, Formatter};

/// Information of a single character.
#[derive(Clone, Copy, CopyGetters, Getters)]
pub struct CharCell<Value> {
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
    pub(crate) value: Value,
}

impl<Value: Display> Display for CharCell<Value> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl<Value: Debug> Debug for CharCell<Value> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let CharCell { value, coord, .. } = self;
        write!(f, "CharCell at {coord} of {value:?}")
    }
}
