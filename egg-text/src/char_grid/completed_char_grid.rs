use super::CharGridLine;
use crate::{
    CharAt, CharCell, CharCoord, CharCount, CharOrEol, LineAt, LineCount, LoadCharAt, LoadLineAt,
    Ordinal, TryIterChar, TryIterLine, TryIterLoadChar, TryIterLoadLine,
};
use derive_more::{Display, Error};
use getset::{CopyGetters, Getters};
use pipe_trait::Pipe;
use std::{cmp::Ordering, convert::Infallible, slice};

/// Character grid with all characters loaded.
#[derive(Clone, CopyGetters, Getters)]
pub struct CompletedCharGrid {
    /// Number of characters.
    #[getset(get_copy = "pub")]
    pub(super) char_count: usize,
    /// Text content.
    #[getset(get = "pub")]
    pub(super) text: String,
    /// List of character cells.
    #[getset(get = "pub")]
    pub(super) char_list: Vec<CharCell<char>>,
    /// List of lines.
    #[getset(get = "pub")]
    pub(super) line_list: Vec<CharGridLine>,
}

/// Error type of [`CharAt`] and [`LoadCharAt`] for [`CompletedCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum CharAtError {
    /// The source iterator doesn't have enough lines to match the requested line index.
    #[display(fmt = "Line does not exist")]
    LineOutOfBound,
    /// The line doesn't have enough characters to match the requested column index.
    #[display(fmt = "Column does not exist")]
    ColumnOutOfBound,
}

impl<'a> CharAt<'a> for CompletedCharGrid {
    type Error = CharAtError;
    fn char_at(&'a self, coord: CharCoord) -> Result<CharCell<char>, CharAtError> {
        let line = self.line_at(coord.column).map_err(|error| match error {
            LineAtError::OutOfBound => CharAtError::LineOutOfBound,
        })?;
        if coord.column.pred_count() >= line.slice().char_count() {
            return Err(CharAtError::ColumnOutOfBound);
        }
        let char_pos = line
            .slice()
            .first_char_pos()
            .advance_by(coord.column.pred_count());
        self.char_list()
            .get(char_pos.pred_count())
            .copied()
            .expect("char_pos should be within the range of char_list")
            .pipe(Ok)
    }
}

impl<'a> LoadCharAt<'a> for CompletedCharGrid {
    type Char = CharCell<char>; // TODO: change this to CharCell<CharOrEol>
    type Error = CharAtError;
    fn load_char_at(&'a mut self, coord: CharCoord) -> Result<CharCell<char>, CharAtError> {
        self.char_at(coord)
    }
}

/// Error type of [`LineAt`] and [`LoadLineAt`] for [`CompletedCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum LineAtError {
    /// The line doesn't have enough characters to match the requested column index.
    #[display(fmt = "Column does not exist")]
    OutOfBound,
}

impl<'a> LineAt<'a> for CompletedCharGrid {
    type Error = LineAtError;
    fn line_at(&'a self, ln_num: Ordinal) -> Result<Self::Line, LineAtError> {
        self.line_list
            .get(ln_num.pred_count())
            .copied()
            .ok_or(LineAtError::OutOfBound)
    }
}

impl<'a> LoadLineAt<'a> for CompletedCharGrid {
    type Line = CharGridLine;
    type Error = LineAtError;
    fn load_line_at(&'a mut self, ln_num: Ordinal) -> Result<Self::Line, Self::Error> {
        self.line_at(ln_num)
    }
}

impl CharCount for CompletedCharGrid {
    fn char_count(&self) -> usize {
        self.char_list().len()
    }
}

impl LineCount for CompletedCharGrid {
    fn line_count(&self) -> usize {
        self.line_list.len()
    }
}

/// An iterator that emits character cells from [`CompletedCharGrid`].
pub struct CharIter<'a> {
    ln_index: Ordinal,
    col_index: Ordinal,
    grid: &'a CompletedCharGrid,
}

impl<'a> Iterator for CharIter<'a> {
    type Item = Result<CharCell<CharOrEol>, Infallible>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.grid.line_at(self.ln_index).ok()?;
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
                    .char_list()
                    .get(char_pos.pred_count())
                    .copied()?
                    .map(CharOrEol::from)
                    .pipe(Ok)
                    .pipe(Some)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let non_eol_count = self.grid.char_list.len();
        let eol_count = self.grid.line_list.len();
        let size = non_eol_count + eol_count;
        (size, Some(size))
    }
}

impl<'a> TryIterChar<'a> for CompletedCharGrid {
    type Error = Infallible;
    type CharResultIter = Self::CharResultLoadIter;
    fn try_iter_char(&'a self) -> Self::CharResultIter {
        CharIter {
            ln_index: Ordinal::from_pred_count(0),
            col_index: Ordinal::from_pred_count(0),
            grid: self,
        }
    }
}

impl<'a> TryIterLoadChar<'a> for CompletedCharGrid {
    type Char = CharCell<CharOrEol>;
    type Error = Infallible;
    type CharResultLoadIter = CharIter<'a>;
    fn try_iter_load_char(&'a mut self) -> Self::CharResultLoadIter {
        self.try_iter_char()
    }
}

/// An iterator that emits lines from a [`CompletedCharGrid`].
pub struct LineIter<'a> {
    iter: slice::Iter<'a, CharGridLine>,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Result<CharGridLine, Infallible>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied().map(Ok)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> TryIterLine<'a> for CompletedCharGrid {
    type Error = Infallible;
    type LineResultIter = Self::LineResultLoadIter;
    fn try_iter_line(&'a self) -> Self::LineResultIter {
        LineIter {
            iter: self.line_list.iter(),
        }
    }
}

impl<'a> TryIterLoadLine<'a> for CompletedCharGrid {
    type Line = CharGridLine;
    type Error = Infallible;
    type LineResultLoadIter = LineIter<'a>;
    fn try_iter_load_line(&'a mut self) -> Self::LineResultLoadIter {
        self.try_iter_line()
    }
}