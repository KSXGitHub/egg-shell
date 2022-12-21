use super::CharGridLine;
use crate::{CharCell, EndOfLine, TextSliceDef};
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
    pub(super) char_list: Vec<CharCell>,
    /// List of lines.
    pub(super) line_list: Vec<(TextSliceDef, EndOfLine)>,
}

impl CompletedCharGrid {
    /// List all loaded lines.
    pub fn line_list(&self) -> impl Iterator<Item = CharGridLine<'_, Self>> {
        let create = |(coord, eol)| CharGridLine::new(coord, eol, self);
        self.line_list.iter().copied().map(create)
    }
}
