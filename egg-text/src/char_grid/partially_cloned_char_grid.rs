mod line;
mod push;

pub use line::*;

use crate::{CharCell, CharOrEol};
use getset::{CopyGetters, Getters};

#[derive(Default, Clone, CopyGetters, Getters)]
pub struct PartiallyClonedCharGrid {
    #[getset(get_copy = "pub")]
    char_count: usize,
    #[getset(get = "pub")]
    text: String,
    #[getset(get = "pub")]
    char_list: Vec<CharCell<CharOrEol>>, // TODO: reduce memory cost by storing only big characters.
    #[getset(get = "pub")]
    line_list: Vec<PartiallyClonedCharGridLineDef>,
}
