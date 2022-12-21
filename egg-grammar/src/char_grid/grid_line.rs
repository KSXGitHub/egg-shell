use super::GridCommon;
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

    /// Get text content of the slice without EOL.
    pub fn text_without_eol(&self) -> &'a str
    where
        CharGrid: GridCommon,
    {
        let start = self.coord.offset();
        let end = start + self.coord.size();
        &self.grid.loaded_text()[start..end]
    }
}

impl<'a, CharGrid> Display for CharGridLine<'a, CharGrid>
where
    CharGrid: GridCommon,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let content = self.text_without_eol();
        let eol = self.eol;
        write!(f, "{content}{eol}")
    }
}

impl<'a, CharGrid> Debug for CharGridLine<'a, CharGrid>
where
    CharGrid: GridCommon,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let content = self.text_without_eol();
        let eol = self.eol;
        write!(f, "CharGridLine {content:?} {eol:?}")
    }
}
