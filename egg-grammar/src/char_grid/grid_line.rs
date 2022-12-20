use super::LazyCharGrid;
use crate::{EndOfLine, TextSliceDef};
use getset::CopyGetters;
use std::fmt::{self, Debug, Display, Formatter};

/// Represent a line in the [`LazyCharGrid`].
#[derive(Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct CharGridLine<'a, CharGrid> {
    /// Coordinate of the line
    coord: TextSliceDef,
    /// Type of EOL string.
    eol: EndOfLine,
    /// Reference grid.
    grid: &'a CharGrid,
}

impl<'a, CharGrid> CharGridLine<'a, CharGrid> {
    /// Create a [`CharGridLine`].
    pub(super) const fn new(coord: TextSliceDef, eol: EndOfLine, grid: &'a CharGrid) -> Self {
        CharGridLine { coord, eol, grid }
    }
}

impl<'a, CharIter> CharGridLine<'a, LazyCharGrid<CharIter>> {
    /// Get text content of the slice without EOL.
    pub fn text_without_eol(&self) -> &'a str {
        let start = self.coord.offset();
        let end = start + self.coord.size();
        &self.grid.loaded_text[start..end]
    }
}

impl<'a, CharIter> Display for CharGridLine<'a, LazyCharGrid<CharIter>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let content = self.text_without_eol();
        let eol = self.eol;
        write!(f, "{content}{eol}")
    }
}

impl<'a, CharIter> Debug for CharGridLine<'a, LazyCharGrid<CharIter>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let content = self.text_without_eol();
        let eol = self.eol;
        write!(f, "CharGridLine {content:?} {eol:?}")
    }
}
