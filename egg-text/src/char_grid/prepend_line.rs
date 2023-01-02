use super::CharGridSliceFrom;
use crate::{CharAt, CharCoord, CharCount, ColumnNumber, LineAt, LineCount, LineNumber, SliceFrom};
use derive_more::{Display, Error};
use std::convert::Infallible;

/// Add a line to before a grid.
#[derive(Debug, Clone, Copy)]
pub struct PrependLine<Head, Tail> {
    /// The line that was added.
    head: Head,
    /// The grid.
    tail: Tail,
}

/// Error type of [`CharAt<CharCoord>`] on [`PrependLine`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum CharAtError<HeadError, TailError> {
    /// Error caused by [`CharAt<Column>`] on the prepended line.
    Head(HeadError),
    /// Error caused by [`CharAt<CharCoord>`] on the grid.
    Tail(TailError),
}

impl<Head, Tail, Char> CharAt<CharCoord> for PrependLine<Head, Tail>
where
    Head: CharAt<ColumnNumber, Char = Char>,
    Tail: CharAt<CharCoord, Char = Char>,
{
    type Char = Char;
    type Error = CharAtError<Head::Error, Tail::Error>;

    fn char_at(self, coord: CharCoord) -> Result<Char, Self::Error> {
        let Some(tail_ln_pred) = coord.line.pred_count().checked_sub(1) else {
            return self.head.char_at(coord.column).map_err(CharAtError::Head)
        };
        let coord = CharCoord::from_pred_counts(tail_ln_pred, coord.column.pred_count());
        self.tail.char_at(coord).map_err(CharAtError::Tail)
    }
}

impl<Head, Tail> LineAt<LineNumber> for PrependLine<Head, Tail>
where
    Tail: LineAt<LineNumber, Line = Head>,
    Head: Copy,
{
    type Line = Head;
    type Error = Tail::Error;

    fn line_at(self, ln_num: LineNumber) -> Result<Head, Self::Error> {
        match ln_num.try_retreat_by(1) {
            None => Ok(self.head),
            Some(tail_ln_num) => self.tail.line_at(tail_ln_num),
        }
    }
}

impl<Head, Tail> SliceFrom<CharCoord> for PrependLine<Head, Tail> {
    type Slice = CharGridSliceFrom<Self, CharCoord>;
    type Error = Infallible;
    fn slice_from(self, start: CharCoord) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl<Head, Tail> CharCount for PrependLine<Head, Tail>
where
    Head: CharCount,
    Tail: CharCount,
{
    fn char_count(&self) -> usize {
        self.head.char_count() + self.tail.char_count()
    }
}

impl<Head, Tail> LineCount for PrependLine<Head, Tail>
where
    Tail: LineCount,
{
    fn line_count(&self) -> usize {
        1 + self.tail.line_count()
    }
}
