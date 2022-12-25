use super::GridCommon;
use crate::{EndOfLine, TextSliceDef};
use getset::CopyGetters;

/// Represent a line in [`super::LazyCharGrid`] and [`super::CompletedCharGrid`].
#[derive(Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct CharGridLine {
    /// Coordinate of the line
    slice: TextSliceDef,
    /// Type of EOL string.
    eol: EndOfLine,
}

impl CharGridLine {
    /// Create a [`CharGridLine`].
    pub(super) const fn new(slice: TextSliceDef, eol: EndOfLine) -> Self {
        CharGridLine { slice, eol }
    }

    /// Get text content of the slice without EOL.
    pub fn text_without_eol<'a, CharGrid>(&self, grid: &'a CharGrid) -> &'a str
    where
        CharGrid: GridCommon,
    {
        let start = self.slice.offset();
        let end = start + self.slice.size();
        &grid.loaded_text()[start..end]
    }
}
