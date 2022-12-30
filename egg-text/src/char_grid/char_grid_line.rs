use super::GridCommon;
use crate::{EndOfLine, TextSliceDef};
use getset::CopyGetters;

/// Represent a line in [`super::LazyCharGrid`] and [`super::CompletedCharGrid`].
#[derive(Debug, Clone, Copy, CopyGetters)]
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
    // TODO: remove grid parameter
    pub fn text_without_eol<'a, CharGrid>(&self, grid: &'a CharGrid) -> CharGrid::Slice
    where
        CharGrid: GridCommon<'a>,
    {
        let start = self.slice.offset();
        let end = start + self.slice.size();
        grid.inner_text_slice(start, end)
    }
}
