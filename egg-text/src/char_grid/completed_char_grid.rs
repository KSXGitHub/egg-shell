use super::{CharGridLine, CharGridSliceFrom};
use crate::{
    CharAt, CharCell, CharCount, CharOrEol, CharPos, ColNum, EndOfLine, LineAt, LineCount, LnCol,
    LnNum, SliceFrom, TextSliceDef, TryIterChar, TryIterLine,
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
    pub(super) char_list: Vec<CharCell<CharOrEol>>, // TODO: reduce memory cost by storing only big characters.
    /// List of lines.
    #[getset(get = "pub")]
    pub(super) line_list: Vec<(TextSliceDef, EndOfLine)>,
}

/// Error type of [`CharAt<LnCol>`] for [`CompletedCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum CharAtLnColError {
    /// The source iterator doesn't have enough lines to match the requested line index.
    #[display(fmt = "Line does not exist")]
    LineOutOfBound,
    /// The line doesn't have enough characters to match the requested column index.
    #[display(fmt = "Column does not exist")]
    ColumnOutOfBound,
}

impl<'a> CharAt<LnCol> for &'a CompletedCharGrid {
    type Char = CharCell<char>;
    type Error = CharAtLnColError;
    fn char_at(self, coord: LnCol) -> Result<CharCell<char>, CharAtLnColError> {
        let line = self.line_at(coord.line).map_err(|error| match error {
            LineAtError::OutOfBound => CharAtLnColError::LineOutOfBound,
        })?;
        if coord.column.pred_count() >= line.slice().char_count() {
            return Err(CharAtLnColError::ColumnOutOfBound);
        }
        let char_pos = line
            .slice()
            .first_char_pos()
            .advance_by(coord.column.pred_count());
        self.char_list()
            .get(char_pos.pred_count())
            .copied()
            .expect("char_pos should be within the range of char_list")
            .pipe(Self::Char::try_from)
            .expect("resulting char should not be an EOL")
            .pipe(Ok)
    }
}

/// Error type of [`CharAt<CharPos>`] for [`CompletedCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum CharAtCharPosError {
    /// The grid doesn't have enough characters to match the requested index.
    #[display(fmt = "Character position does not exist")]
    OutOfBound,
}

impl<'a> CharAt<CharPos> for &'a CompletedCharGrid {
    type Char = CharCell<CharOrEol>;
    type Error = CharAtCharPosError;
    fn char_at(self, pos: CharPos) -> Result<Self::Char, CharAtCharPosError> {
        self.char_list()
            .get(pos.pred_count())
            .copied()
            .ok_or(CharAtCharPosError::OutOfBound)
    }
}

/// Error type of [`LineAt`] for [`CompletedCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum LineAtError {
    /// The line doesn't have enough characters to match the requested column index.
    #[display(fmt = "Column does not exist")]
    OutOfBound,
}

impl<'a> LineAt<LnNum> for &'a CompletedCharGrid {
    type Line = CharGridLine<Self>;
    type Error = LineAtError;
    fn line_at(self, ln_num: LnNum) -> Result<Self::Line, LineAtError> {
        self.line_list
            .get(ln_num.pred_count())
            .map(|&(slice, eol)| CharGridLine::new(slice, eol, self))
            .ok_or(LineAtError::OutOfBound)
    }
}

impl<'a> SliceFrom<LnNum> for &'a CompletedCharGrid {
    type Slice = CharGridSliceFrom<Self, LnNum>;
    type Error = Infallible;
    fn slice_from(self, start: LnNum) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl<'a> SliceFrom<LnCol> for &'a CompletedCharGrid {
    type Slice = CharGridSliceFrom<Self, LnCol>;
    type Error = Infallible;
    fn slice_from(self, start: LnCol) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl CharCount for CompletedCharGrid {
    fn char_count(&self) -> usize {
        let non_eol = self.char_list().len();
        let eol = self.line_list.len();
        non_eol + eol
    }
}

impl LineCount for CompletedCharGrid {
    fn line_count(&self) -> usize {
        self.line_list.len()
    }
}

/// An iterator that emits character cells from [`CompletedCharGrid`].
pub struct CharIter<'a> {
    ln_index: LnNum,
    col_index: ColNum,
    grid: &'a CompletedCharGrid,
}

impl<'a> Iterator for CharIter<'a> {
    type Item = Result<CharCell<CharOrEol>, Infallible>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.grid.line_at(self.ln_index).ok()?;
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
        let max = self.grid.text.len();
        let min = max / std::mem::size_of::<char>();
        (min, Some(max))
    }
}

impl<'a> TryIterChar for &'a CompletedCharGrid {
    type Char = CharCell<CharOrEol>;
    type Error = Infallible;
    type CharResultIter = CharIter<'a>;
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

/// An iterator that emits lines from a [`CompletedCharGrid`].
pub struct LineIter<'a> {
    iter: slice::Iter<'a, (TextSliceDef, EndOfLine)>,
    grid: &'a CompletedCharGrid,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Result<CharGridLine<&'a CompletedCharGrid>, Infallible>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|&(slice, eol)| CharGridLine::new(slice, eol, self.grid))
            .map(Ok)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> TryIterLine for &'a CompletedCharGrid {
    type Line = CharGridLine<Self>;
    type Error = Infallible;
    type LineResultIter = LineIter<'a>;
    fn try_iter_line(self) -> Self::LineResultIter {
        LineIter {
            iter: self.line_list.iter(),
            grid: self,
        }
    }
}
