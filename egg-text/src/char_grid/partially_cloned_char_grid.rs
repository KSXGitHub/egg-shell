use crate::{CharCell, CharOrEol, EndOfLine, TextSliceDef};
use getset::{CopyGetters, Getters};

#[derive(Clone, CopyGetters, Getters)]
pub struct PartiallyClonedCharGrid {
    #[getset(get_copy = "pub")]
    pub(super) char_count: usize,
    #[getset(get = "pub")]
    pub(super) text: String,
    #[getset(get = "pub")]
    pub(super) char_list: Vec<CharCell<CharOrEol>>, // TODO: reduce memory cost by storing only big characters.
    #[getset(get = "pub")]
    pub(super) line_list: Vec<(TextSliceDef, EndOfLine)>,
}
