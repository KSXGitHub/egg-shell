mod build;
mod push;

use crate::{CharCell, CharOrEol, EndOfLine, TextSliceDef};
use derive_more::Display;
use getset::{CopyGetters, Getters};

/// Builder of [`PartiallyClonedCharGrid`].
#[derive(CopyGetters, Getters)]
pub struct PartiallyClonedCharGridBuilder {
    /// Number of characters.
    #[getset(get_copy = "pub")]
    char_count: usize,
    /// Text content.
    #[getset(get = "pub")]
    text: String,
    /// List of character cells.
    #[getset(get = "pub")]
    char_list: Vec<CharCell<CharOrEol>>, // TODO: reduce memory cost by storing only big characters.
    /// List of lines.
    #[getset(get = "pub")]
    line_list: Vec<(TextSliceDef, EndOfLine)>,
    /// Current line cursor.
    #[getset(get_copy = "pub")]
    current_ln_start: usize,
}

/// A grid that is a clone of a section of another grid.
#[derive(Display, Default, Clone, CopyGetters, Getters)]
#[display(fmt = "{text}")]
pub struct PartiallyClonedCharGrid {
    /// Number of characters.
    #[getset(get_copy = "pub")]
    char_count: usize,
    /// Text content.
    #[getset(get = "pub")]
    text: String,
    /// List of character cells.
    #[getset(get = "pub")]
    char_list: Vec<CharCell<CharOrEol>>, // TODO: reduce memory cost by storing only big characters.
    /// List of lines.
    #[getset(get = "pub")]
    line_list: Vec<(TextSliceDef, EndOfLine)>,
}
