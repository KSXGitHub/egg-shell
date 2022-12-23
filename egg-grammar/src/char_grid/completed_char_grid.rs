use super::CharGridLine;
use crate::{
    CharAt, CharCell, CharCoord, CharCount, EndOfLine, LineAt, LineCount, LoadCharAt, LoadLineAt,
    Ordinal, TextSliceDef, TryIterChar, TryIterLine, TryIterLoadChar, TryIterLoadLine,
};
use getset::{CopyGetters, Getters};
use pipe_trait::Pipe;
use std::{convert::Infallible, slice};
use thiserror::Error;

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
    pub(super) char_list: Vec<CharCell>,
    /// List of lines.
    pub(super) line_list: Vec<(TextSliceDef, EndOfLine)>,
}

impl CompletedCharGrid {
    /// List all loaded lines.
    pub fn line_list(&self) -> impl Iterator<Item = CharGridLine<'_, Self>> {
        let create = |(coord, eol)| CharGridLine::new(coord, eol, self);
        self.line_list.iter().copied().map(create)
    }
}

/// Error type of [`CharAt`] and [`LoadCharAt`] for [`CompletedCharGrid`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum CharAtError {
    #[error("Line does not exist")]
    LineOutOfBound,
    #[error("Column does not exist")]
    ColumnOutOfBound,
}

impl<'a> CharAt<'a> for CompletedCharGrid {
    type Error = CharAtError;
    fn char_at(&'a self, coord: CharCoord) -> Result<CharCell, CharAtError> {
        let line = self.line_at(coord.column).map_err(|error| match error {
            LineAtError::OutOfBound => CharAtError::LineOutOfBound,
        })?;
        if coord.column.pred_count() > line.slice().char_count() {
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
    type Error = CharAtError;
    fn load_char_at(&'a mut self, coord: CharCoord) -> Result<CharCell, CharAtError> {
        self.char_at(coord)
    }
}

/// Error type of [`LineAt`] and [`LoadLineAt`] for [`CompletedCharGrid`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum LineAtError {
    #[error("Column does not exist")]
    OutOfBound,
}

impl<'a> LineAt<'a> for CompletedCharGrid {
    type Error = LineAtError;
    fn line_at(&'a self, ln_num: Ordinal) -> Result<Self::Line, LineAtError> {
        let (line, eol) = *self
            .line_list
            .get(ln_num.pred_count())
            .ok_or(LineAtError::OutOfBound)?;
        CharGridLine::new(line, eol, self).pipe(Ok)
    }
}

impl<'a> LoadLineAt<'a> for CompletedCharGrid {
    type Line = CharGridLine<'a, CompletedCharGrid>;
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
pub struct CharIter<'a>(slice::Iter<'a, CharCell>);

impl<'a> Iterator for CharIter<'a> {
    type Item = Result<CharCell, Infallible>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied()?.pipe(Ok).pipe(Some)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> TryIterChar<'a> for CompletedCharGrid {
    type Error = Infallible;
    type CharResultIter = Self::CharResultLoadIter;
    fn try_iter_char(&'a self) -> Self::CharResultIter {
        self.char_list().iter().pipe(CharIter)
    }
}

impl<'a> TryIterLoadChar<'a> for CompletedCharGrid {
    type Error = Infallible;
    type CharResultLoadIter = CharIter<'a>;
    fn try_iter_load_char(&'a mut self) -> Self::CharResultLoadIter {
        self.try_iter_char()
    }
}

/// An iterator that emits lines from a [`CompletedCharGrid`].
pub struct LineIter<'a> {
    iter: slice::Iter<'a, (TextSliceDef, EndOfLine)>,
    grid: &'a CompletedCharGrid,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Result<CharGridLine<'a, CompletedCharGrid>, Infallible>;

    fn next(&mut self) -> Option<Self::Item> {
        let (slice, eol) = *self.iter.next()?;
        CharGridLine::new(slice, eol, self.grid).pipe(Ok).pipe(Some)
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
            grid: self,
        }
    }
}

impl<'a> TryIterLoadLine<'a> for CompletedCharGrid {
    type Line = CharGridLine<'a, Self>;
    type Error = Infallible;
    type LineResultLoadIter = LineIter<'a>;
    fn try_iter_load_line(&'a mut self) -> Self::LineResultLoadIter {
        self.try_iter_line()
    }
}
