use super::GridCommon;
use crate::{EndOfLine, TextSliceDef};
use getset::CopyGetters;
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::Deref,
};

/// Represent a line in [`super::LazyCharGrid`] and [`super::CompletedCharGrid`].
#[derive(Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct CharGridLine<CharGridRef: Copy> {
    /// Coordinate of the line
    slice: TextSliceDef,
    /// Type of EOL string.
    eol: EndOfLine,
    /// Reference grid.
    grid: CharGridRef,
}

impl<CharGridRef: Copy> CharGridLine<CharGridRef> {
    /// Create a [`CharGridLine`].
    pub(super) const fn new(slice: TextSliceDef, eol: EndOfLine, grid: CharGridRef) -> Self {
        CharGridLine { slice, eol, grid }
    }

    /// Get text content of the slice without EOL.
    pub fn text_without_eol<'a>(&'a self) -> <CharGridRef::Target as GridCommon>::Slice
    where
        CharGridRef: Deref,
        CharGridRef::Target: GridCommon<'a>,
    {
        let start = self.slice.offset();
        let end = start + self.slice.size();
        self.grid.inner_text_slice(start, end)
    }
}

impl<CharGridRef> Debug for CharGridLine<CharGridRef>
where
    CharGridRef: Deref + Copy,
    CharGridRef::Target: for<'r> GridCommon<'r>,
    for<'r> <CharGridRef::Target as GridCommon<'r>>::Slice: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let text = self.text_without_eol();
        let eol = self.eol;
        write!(f, "CharGridLine {text:?} {eol:?}")
    }
}

impl<CharGridRef> Display for CharGridLine<CharGridRef>
where
    CharGridRef: Deref + Copy,
    CharGridRef::Target: for<'r> GridCommon<'r>,
    for<'r> <CharGridRef::Target as GridCommon<'r>>::Slice: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let text = self.text_without_eol();
        let eol = self.eol;
        write!(f, "{text}{eol}")
    }
}
