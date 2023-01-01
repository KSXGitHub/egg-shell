use super::{CharGridSliceFrom, GridCommon};
use crate::{CharAt, CharCoord, ColumnNumber, EndOfLine, SliceFrom, TextSliceDef};
use getset::CopyGetters;
use std::{
    convert::Infallible,
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

impl<'a, CharGridRef> CharAt<ColumnNumber> for &'a CharGridLine<CharGridRef>
where
    CharGridRef: Copy + CharAt<CharCoord> + 'a,
{
    type Char = CharGridRef::Char;
    type Error = CharGridRef::Error;

    fn char_at(self, col_num: ColumnNumber) -> Result<Self::Char, Self::Error> {
        let coord = self
            .slice
            .first_char_coord()
            .advance_column(col_num.pred_count());
        self.grid.char_at(coord)
    }
}

impl<'a, CharGridRef> SliceFrom<'a, ColumnNumber> for CharGridLine<CharGridRef>
where
    CharGridRef: Copy + 'a,
{
    type Slice = CharGridSliceFrom<&'a Self, ColumnNumber>;
    type Error = Infallible;

    fn slice_from(&'a self, start: ColumnNumber) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}
