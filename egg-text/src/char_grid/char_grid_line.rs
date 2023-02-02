use super::{CharGridSliceFrom, GridCommon};
use crate::{
    CharAt, CharPos, CharPosOutOfBound, ColNum, EndOfLine, LnCol, SliceFrom, TextSliceSep,
    TryIterChar,
};
use derive_more::{Display, Error};
use getset::CopyGetters;
use std::{
    convert::Infallible,
    fmt::{self, Debug, Display, Formatter},
};

/// Represent a line in [`super::LazyCharGrid`] and [`super::CompletedCharGrid`].
#[derive(Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct CharGridLine<CharGridRef: Copy> {
    /// Coordinate of the first character of the line.
    pub(crate) first_char_coord: LnCol,
    /// Coordinate of the beginning of the line
    pub(crate) start: TextSliceSep,
    /// Coordinate of the end of the line
    pub(crate) end: TextSliceSep,
    /// Type of EOL string.
    pub(crate) eol: EndOfLine,
    /// Reference grid.
    pub(crate) grid: CharGridRef,
}

impl<CharGridRef: Copy> CharGridLine<CharGridRef> {
    /// Size of the line in bytes, not including EOL.
    pub fn size_wo_eol(&self) -> usize {
        self.end.offset() - self.start.offset()
    }

    /// Number of non-EOL characters in the line.
    pub fn char_count_wo_eol(&self) -> usize {
        self.end.index().pred_count() - self.start.index().pred_count()
    }

    /// Get text content of the slice without EOL.
    pub fn text_without_eol(&self) -> CharGridRef::Slice
    where
        CharGridRef: GridCommon,
    {
        let start = self.start.offset();
        let end = self.end.offset();
        self.grid.inner_text_slice(start, end)
    }
}

impl<CharGridRef> Debug for CharGridLine<CharGridRef>
where
    CharGridRef: GridCommon + Copy,
    CharGridRef::Slice: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let text = self.text_without_eol();
        let eol = self.eol;
        write!(f, "CharGridLine {text:?} {eol:?}")
    }
}

impl<CharGridRef> Display for CharGridLine<CharGridRef>
where
    CharGridRef: GridCommon + Copy,
    CharGridRef::Slice: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let text = self.text_without_eol();
        let eol = self.eol;
        write!(f, "{text}{eol}")
    }
}

impl<CharGridRef> CharAt<ColNum> for CharGridLine<CharGridRef>
where
    CharGridRef: CharAt<LnCol> + Copy,
{
    type Char = CharGridRef::Char;
    type Error = CharGridRef::Error;

    fn char_at(self, col_num: ColNum) -> Result<Self::Char, Self::Error> {
        let coord = self.first_char_coord().advance_column(col_num.pred_count());
        self.grid.char_at(coord)
    }
}

/// Error type of [`CharAt<CharPos>`] on [`CharGridLine`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum ChatAtCharPosError<GridError> {
    /// Error propagated from [`CharAt`] on the grid.
    GridError(GridError),
    /// The requested index is greater than the bound of the line.
    #[display(fmt = "Character position does not exist")]
    OutOfBound,
}

impl<GridError> TryFrom<ChatAtCharPosError<GridError>> for CharPosOutOfBound {
    type Error = GridError;
    fn try_from(value: ChatAtCharPosError<GridError>) -> Result<Self, Self::Error> {
        match value {
            ChatAtCharPosError::GridError(error) => Err(error),
            ChatAtCharPosError::OutOfBound => Ok(CharPosOutOfBound),
        }
    }
}

impl<CharGridRef> CharAt<CharPos> for CharGridLine<CharGridRef>
where
    CharGridRef: CharAt<CharPos> + Copy,
{
    type Char = CharGridRef::Char;
    type Error = ChatAtCharPosError<CharGridRef::Error>;

    fn char_at(self, pos: CharPos) -> Result<Self::Char, Self::Error> {
        if pos > self.end().index() {
            return Err(ChatAtCharPosError::OutOfBound);
        }
        let pos = self.start().index().advance_by(pos.pred_count());
        self.grid
            .char_at(pos)
            .map_err(ChatAtCharPosError::GridError)
    }
}

impl<CharGridRef> SliceFrom<ColNum> for CharGridLine<CharGridRef>
where
    CharGridRef: Copy,
{
    type Slice = CharGridSliceFrom<Self, ColNum>;
    type Error = Infallible;

    fn slice_from(self, start: ColNum) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl<CharGridRef> SliceFrom<CharPos> for CharGridLine<CharGridRef>
where
    CharGridRef: Copy,
{
    type Slice = CharGridSliceFrom<Self, CharPos>;
    type Error = Infallible;

    fn slice_from(self, start: CharPos) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

/// Character iterator of [`CharGridLine`].
pub struct CharIter<CharGridRef>
where
    CharGridRef: CharAt<CharPos> + Copy,
{
    index: usize,
    line: CharGridLine<CharGridRef>,
}

impl<CharGridRef> Iterator for CharIter<CharGridRef>
where
    CharGridRef: CharAt<CharPos> + Copy,
{
    type Item = Result<CharGridRef::Char, CharGridRef::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let CharIter { index, line } = self;
        let CharGridLine {
            start, end, grid, ..
        } = *line;
        let pos = start.index().advance_by(*index);
        if pos > end.index() {
            return None;
        }
        *index += 1;
        Some(grid.char_at(pos))
    }
}

impl<CharGridRef> TryIterChar for CharGridLine<CharGridRef>
where
    CharGridRef: CharAt<CharPos> + Copy,
{
    type Char = CharGridRef::Char;
    type Error = CharGridRef::Error;
    type CharResultIter = CharIter<CharGridRef>;

    fn try_iter_char(self) -> Self::CharResultIter {
        CharIter {
            index: 0,
            line: self,
        }
    }
}

/// Template of [`CharGridLine`].
pub type CharGridLineTemplate = CharGridLine<()>;

impl Debug for CharGridLineTemplate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharGridLineTemplate")
            .field("first_char_coord", &self.first_char_coord)
            .field("start", &self.start)
            .field("end", &self.end)
            .field("eol", &self.eol)
            .finish()
    }
}
