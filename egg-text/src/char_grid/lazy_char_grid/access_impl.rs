use super::{LazyCharGrid, LoadCharError};
use crate::{
    char_grid::CharGridLine, CharCell, CharCoord, CharOrEol, LoadCharAt, LoadLineAt, Ordinal,
    TryIterLoadChar, TryIterLoadLine,
};
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use std::{cmp::Ordering, fmt::Debug};

/// Error type of [`LoadCharAt`] for [`LazyCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum CharAtError<IterError> {
    /// An error occurred while loading a character.
    LoadCharError(LoadCharError<IterError>),
    /// The source iterator doesn't have enough lines to match the requested line index.
    #[display(fmt = "Line does not exist")]
    LineOutOfBound,
    /// The line doesn't have enough characters to match the requested column index.
    #[display(fmt = "Column does not exist")]
    ColumnOutOfBound,
}

impl<'a, IterError, CharIter> LoadCharAt<'a> for LazyCharGrid<CharIter>
where
    CharIter: Iterator<Item = Result<char, IterError>>,
{
    type Char = CharCell<char>; // TODO: change this to CharCell<CharOrEol>
    type Error = CharAtError<IterError>;

    fn load_char_at(&'a mut self, coord: CharCoord) -> Result<CharCell<char>, Self::Error> {
        let line = self.load_line_at(coord.line).map_err(|error| match error {
            LineAtError::LoadCharError(error) => CharAtError::LoadCharError(error),
            LineAtError::OutOfBound => CharAtError::LineOutOfBound,
        })?;
        if coord.column.pred_count() >= line.slice().char_count() {
            return Err(CharAtError::ColumnOutOfBound);
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
            .pipe(Ok)
    }
}

/// Error type of [`LoadLineAt`] for [`LazyCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum LineAtError<IterError> {
    /// An error occurred while loading a character.
    LoadCharError(LoadCharError<IterError>),
    /// The source iterator doesn't have enough lines to match the requested index.
    #[display(fmt = "Line does not exist")]
    OutOfBound,
}

impl<'a, IterError, CharIter> LoadLineAt<'a> for LazyCharGrid<CharIter>
where
    CharIter: Iterator<Item = Result<char, IterError>> + 'a,
{
    type Error = LineAtError<IterError>;
    type Line = CharGridLine;
    fn load_line_at(&'a mut self, ln_num: Ordinal) -> Result<Self::Line, Self::Error> {
        while self.data().loaded_line_list.len() <= ln_num.pred_count()
            && self.completion().is_incomplete()
        {
            self.load_line().map_err(LineAtError::LoadCharError)?;
        }
        if let Some(line) = self.data().loaded_line_list.get(ln_num.pred_count()) {
            return Ok(*line);
        }
        Err(LineAtError::OutOfBound)
    }
}

/// An iterator that emits instances of [`CharCell`] from [`LazyCharGrid`].
pub struct CharIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    ln_index: Ordinal,
    col_index: Ordinal,
    grid: &'a mut LazyCharGrid<SrcIter>,
}

impl<'a, SrcIterError, SrcIter> Iterator for CharIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Item = Result<CharCell<CharOrEol>, LoadCharError<SrcIterError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = match self.grid.load_line_at(self.ln_index) {
            Err(LineAtError::LoadCharError(error)) => return Some(Err(error)),
            Err(LineAtError::OutOfBound) => return None,
            Ok(line) => line,
        };
        match self.col_index.pred_count().cmp(&line.slice().char_count()) {
            Ordering::Greater => panic!("Column index should never be greater than line count"),
            Ordering::Equal => {
                let coord = CharCoord {
                    line: self.ln_index,
                    column: self.col_index,
                };
                self.ln_index = self.ln_index.advance_by(1);
                self.col_index = Ordinal::from_pred_count(0);
                let offset_from_ln_start = line.slice().size();
                let offset_from_doc_start = line.slice().offset() + line.slice().size();
                let value = CharOrEol::EndOfLine(line.eol());
                let char_cell = CharCell {
                    coord,
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

impl<'a, SrcIterError, SrcIter> TryIterLoadChar<'a> for LazyCharGrid<SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Char = CharCell<CharOrEol>;
    type Error = LoadCharError<SrcIterError>;
    type CharResultLoadIter = CharIter<'a, SrcIterError, SrcIter>;

    fn try_iter_load_char(&'a mut self) -> Self::CharResultLoadIter {
        CharIter {
            ln_index: Ordinal::from_pred_count(0),
            col_index: Ordinal::from_pred_count(0),
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
    grid: &'a mut LazyCharGrid<SrcIter>,
}

impl<'a, SrcIterError, SrcIter> Iterator for LineIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Item = Result<CharGridLine, LoadCharError<SrcIterError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index = index.advance_by(1);
        let line = self
            .grid
            .load_line_at(Ordinal::from_pred_count(index.pred_count()));
        match line {
            Err(LineAtError::LoadCharError(error)) => Some(Err(error)),
            Err(LineAtError::OutOfBound) => None,
            Ok(line) => Some(Ok(line)),
        }
    }
}

impl<'a, SrcIterError, SrcIter> TryIterLoadLine<'a> for LazyCharGrid<SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Line = CharGridLine;
    type Error = LoadCharError<SrcIterError>;
    type LineResultLoadIter = LineIter<'a, SrcIterError, SrcIter>;

    fn try_iter_load_line(&'a mut self) -> Self::LineResultLoadIter {
        LineIter {
            index: Ordinal::from_pred_count(0),
            grid: self,
        }
    }
}
