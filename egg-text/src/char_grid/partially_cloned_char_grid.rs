mod line;
mod push;

pub use line::*;

use crate::{CharCell, CharOrEol};
use derive_more::Display;
use getset::{CopyGetters, Getters};

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
    /// List of EOL markers.
    #[getset(get = "pub")]
    eol_list: Vec<PartiallyClonedCharGridEol>,
}
