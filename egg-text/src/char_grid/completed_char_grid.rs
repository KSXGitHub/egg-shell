mod impl_char_at_char_pos;
mod impl_char_at_ln_col;
mod impl_count;
mod impl_line_at_ln_num;
mod impl_slice_from;
mod impl_try_iter_char;
mod impl_try_iter_line;

pub use impl_char_at_char_pos::*;
pub use impl_char_at_ln_col::*;
pub use impl_line_at_ln_num::*;
pub use impl_try_iter_char::*;
pub use impl_try_iter_line::*;

use crate::{CharCell, CharOrEol, EndOfLine, TextSliceDef};
use getset::{CopyGetters, Getters};

/// Character grid with all characters loaded.
#[derive(Clone, CopyGetters, Getters)]
pub struct CompletedCharGrid {
    /// Number of characters.
    #[getset(get_copy = "pub")]
    pub(super) char_count: usize,
    /// Text content.
    #[getset(get = "pub")]
    pub(super) text: String,
    /// List of character cells.
    #[getset(get = "pub")]
    pub(super) char_list: Vec<CharCell<CharOrEol>>, // TODO: reduce memory cost by storing only big characters.
    /// List of lines.
    #[getset(get = "pub")]
    pub(super) line_list: Vec<(TextSliceDef, EndOfLine)>,
}
