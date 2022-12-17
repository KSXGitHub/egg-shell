use crate::CharCoord;
use derive_more::Display;
use getset::CopyGetters;
use std::fmt::{self, Debug, Formatter};

/// Information of a single character.
#[derive(Display, Clone, Copy, CopyGetters)]
#[display(fmt = "{value}")]
#[getset(get_copy = "pub")]
pub struct CharCell {
    /// Character coordinate.
    pub(crate) coord: CharCoord,
    /// Byte offset from the start of the line.
    pub(crate) offset_from_ln_start: usize,
    /// Byte offset from the start of the document.
    pub(crate) offset_from_doc_start: usize,
    /// Content of the character.
    pub(crate) value: char,
}

impl Debug for CharCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let CharCell { value, coord, .. } = self;
        write!(f, "CharCell at {coord} of {value:?}")
    }
}
