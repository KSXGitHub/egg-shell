use super::{LazyCharGrid, LoadCharError};
use crate::{
    char_grid::{CharGridLine, CharGridSliceFrom},
    CharAt, CharCell, CharOrEol, CharPos, CharPosOutOfBound, ColNum, LineAt, LnCol,
    LnColOutOfBound, LnNum, LnNumOutOfBound, Ordinal, SliceFrom, TryIterChar, TryIterLine,
};
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use std::{cmp::Ordering, convert::Infallible, fmt::Debug};

/// Error type of [`CharAt<LnCol>`] for [`LazyCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum CharAtLnColError<IterError> {
    /// An error occurred while loading a character.
    LoadCharError(LoadCharError<IterError>),
    /// The source iterator doesn't have enough lines to match the requested line index.
    #[display(fmt = "Line does not exist")]
    LineOutOfBound,
    /// The line doesn't have enough characters to match the requested column index.
    #[display(fmt = "Column does not exist")]
    ColumnOutOfBound,
}

impl<IterError> TryFrom<CharAtLnColError<IterError>> for LnColOutOfBound {
    type Error = LoadCharError<IterError>;
    fn try_from(value: CharAtLnColError<IterError>) -> Result<Self, Self::Error> {
        match value {
            CharAtLnColError::LoadCharError(error) => Err(error),
            CharAtLnColError::LineOutOfBound => Ok(LnColOutOfBound::LineOutOfBound),
            CharAtLnColError::ColumnOutOfBound => Ok(LnColOutOfBound::ColumnOutOfBound),
        }
    }
}

impl<'a, IterError, CharIter> CharAt<LnCol> for &'a LazyCharGrid<CharIter>
where
    CharIter: Iterator<Item = Result<char, IterError>>,
{
    type Char = CharCell<char>;
    type Error = CharAtLnColError<IterError>;

    fn char_at(self, coord: LnCol) -> Result<CharCell<char>, CharAtLnColError<IterError>> {
        let line = self.line_at(coord.line).map_err(|error| match error {
            LineAtError::LoadCharError(error) => CharAtLnColError::LoadCharError(error),
            LineAtError::OutOfBound => CharAtLnColError::LineOutOfBound,
        })?;
        if coord.column.pred_count() >= line.slice().char_count() {
            return Err(CharAtLnColError::ColumnOutOfBound);
        }
        let char_pos = line
            .slice()
            .first_char_pos()
            .advance_by(coord.column.pred_count());
        self.data()
            .loaded_char_list()
            .get(char_pos.pred_count())
            .copied()
            .expect("char_pos should be within the range of char_list")
            .pipe(Self::Char::try_from)
            .expect("resulting char should not be an EOL")
            .pipe(Ok)
    }
}

/// Error type of [`CharAt<CharPos>`] for [`LazyCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum CharAtCharPosError<IterError> {
    /// An error occurred while loading a character.
    LoadCharError(LoadCharError<IterError>),
    /// The grid doesn't have enough characters to match the requested index.
    #[display(fmt = "Character position does not exist")]
    OutOfBound,
}

impl<IterError> TryFrom<CharAtCharPosError<IterError>> for CharPosOutOfBound {
    type Error = LoadCharError<IterError>;
    fn try_from(value: CharAtCharPosError<IterError>) -> Result<Self, Self::Error> {
        match value {
            CharAtCharPosError::LoadCharError(error) => Err(error),
            CharAtCharPosError::OutOfBound => Ok(CharPosOutOfBound),
        }
    }
}

impl<'a, IterError, CharIter> CharAt<CharPos> for &'a LazyCharGrid<CharIter>
where
    CharIter: Iterator<Item = Result<char, IterError>>,
{
    type Char = CharCell<CharOrEol>;
    type Error = CharAtCharPosError<IterError>;

    fn char_at(self, pos: CharPos) -> Result<Self::Char, Self::Error> {
        while self.completion().is_incomplete() && pos.pred_count() >= self.loaded_char_count() {
            self.load_char()
                .map_err(CharAtCharPosError::LoadCharError)?;
        }
        self.data()
            .loaded_char_list()
            .get(pos.pred_count())
            .copied()
            .ok_or(CharAtCharPosError::OutOfBound)
    }
}

/// Error type of [`LineAt`] for [`LazyCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum LineAtError<IterError> {
    /// An error occurred while loading a character.
    LoadCharError(LoadCharError<IterError>),
    /// The source iterator doesn't have enough lines to match the requested index.
    #[display(fmt = "Line does not exist")]
    OutOfBound,
}

impl<IterError> TryFrom<LineAtError<IterError>> for LnNumOutOfBound {
    type Error = LoadCharError<IterError>;
    fn try_from(value: LineAtError<IterError>) -> Result<Self, Self::Error> {
        match value {
            LineAtError::LoadCharError(error) => Err(error),
            LineAtError::OutOfBound => Ok(LnNumOutOfBound),
        }
    }
}

impl<'a, IterError, CharIter> LineAt<LnNum> for &'a LazyCharGrid<CharIter>
where
    CharIter: Iterator<Item = Result<char, IterError>> + 'a,
{
    type Line = CharGridLine<Self>;
    type Error = LineAtError<IterError>;
    fn line_at(self, ln_num: LnNum) -> Result<Self::Line, LineAtError<IterError>> {
        while self.data().loaded_line_list.len() <= ln_num.pred_count()
            && self.completion().is_incomplete()
        {
            self.load_line().map_err(LineAtError::LoadCharError)?;
        }
        if let Some((slice, eol)) = self.data().loaded_line_list.get(ln_num.pred_count()) {
            return Ok(CharGridLine::new(*slice, *eol, self));
        }
        Err(LineAtError::OutOfBound)
    }
}

impl<'a, SrcIter: 'a> SliceFrom<LnNum> for &'a LazyCharGrid<SrcIter> {
    type Slice = CharGridSliceFrom<Self, LnNum>;
    type Error = Infallible;
    fn slice_from(self, start: LnNum) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl<'a, SrcIter: 'a> SliceFrom<LnCol> for &'a LazyCharGrid<SrcIter> {
    type Slice = CharGridSliceFrom<Self, LnCol>;
    type Error = Infallible;
    fn slice_from(self, start: LnCol) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl<'a, SrcIter: 'a> SliceFrom<CharPos> for &'a LazyCharGrid<SrcIter> {
    type Slice = CharGridSliceFrom<Self, CharPos>;
    type Error = Infallible;
    fn slice_from(self, start: CharPos) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

/// An iterator that emits instances of [`CharCell`] from [`LazyCharGrid`].
pub struct CharIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    ln_index: LnNum,
    col_index: ColNum,
    grid: &'a LazyCharGrid<SrcIter>,
}

impl<'a, SrcIterError, SrcIter> Iterator for CharIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Item = Result<CharCell<CharOrEol>, LoadCharError<SrcIterError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = match self.grid.line_at(self.ln_index) {
            Err(LineAtError::LoadCharError(error)) => return Some(Err(error)),
            Err(LineAtError::OutOfBound) => return None,
            Ok(line) => line,
        };
        match self.col_index.pred_count().cmp(&line.slice().char_count()) {
            Ordering::Greater => panic!("Column index should never be greater than line count"),
            Ordering::Equal => {
                let coord = LnCol {
                    line: self.ln_index,
                    column: self.col_index,
                };
                let col_count = coord.column.pred_count(); // number of columns from line start
                let pos = line.slice().first_char_pos().advance_by(col_count);
                self.ln_index = self.ln_index.advance_by(1);
                self.col_index = ColNum::from_pred_count(0);
                let offset_from_ln_start = line.slice().size();
                let offset_from_doc_start = line.slice().offset() + line.slice().size();
                let value = CharOrEol::EndOfLine(line.eol());
                let char_cell = CharCell {
                    coord,
                    pos,
                    offset_from_ln_start,
                    offset_from_doc_start,
                    value,
                };
                Some(Ok(char_cell))
            }
            Ordering::Less => {
                let char_pos = line
                    .slice()
                    .first_char_pos()
                    .advance_by(self.col_index.pred_count());
                self.col_index = self.col_index.advance_by(1);
                self.grid
                    .data()
                    .loaded_char_list()
                    .get(char_pos.pred_count())
                    .copied()?
                    .map(CharOrEol::from)
                    .pipe(Ok)
                    .pipe(Some)
            }
        }
    }
}

impl<'a, SrcIterError, SrcIter> TryIterChar for &'a LazyCharGrid<SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Char = CharCell<CharOrEol>;
    type Error = LoadCharError<SrcIterError>;
    type CharResultIter = CharIter<'a, SrcIterError, SrcIter>;

    fn try_iter_char(self) -> Self::CharResultIter {
        // Q: Why not just iterate over existing self.data.char_list?
        // A: data.char_list is very expensive in term of memory, it is to be removed in the future.
        CharIter {
            ln_index: LnNum::from_pred_count(0),
            col_index: ColNum::from_pred_count(0),
            grid: self,
        }
    }
}

/// An iterator that emits instances of [`CharGridLine`] from [`LazyCharGrid`].
pub struct LineIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    index: Ordinal,
    grid: &'a LazyCharGrid<SrcIter>,
}

impl<'a, SrcIterError, SrcIter> Iterator for LineIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Item = Result<CharGridLine<&'a LazyCharGrid<SrcIter>>, LoadCharError<SrcIterError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index = index.advance_by(1);
        let line = self
            .grid
            .line_at(LnNum::from_pred_count(index.pred_count()));
        match line {
            Err(LineAtError::LoadCharError(error)) => Some(Err(error)),
            Err(LineAtError::OutOfBound) => None,
            Ok(line) => Some(Ok(line)),
        }
    }
}

impl<'a, SrcIterError, SrcIter> TryIterLine for &'a LazyCharGrid<SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Line = CharGridLine<Self>;
    type Error = LoadCharError<SrcIterError>;
    type LineResultIter = LineIter<'a, SrcIterError, SrcIter>;

    fn try_iter_line(self) -> Self::LineResultIter {
        LineIter {
            index: Ordinal::from_pred_count(0),
            grid: self,
        }
    }
}
