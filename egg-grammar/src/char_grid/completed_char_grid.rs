use super::CharGridLine;
use crate::{
    CharAt, CharCell, CharCoord, CharCount, EndOfLine, IterChar, IterLine, IterLoadChar,
    IterLoadLine, LineAt, LoadCharAt, LoadLineAt, Ordinal, TextSliceDef,
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
        let (line, _) = *self
            .line_list
            .get(coord.line.pred_count())
            .ok_or(CharAtError::LineOutOfBound)?;
        if coord.column.pred_count() > line.char_count() {
            return Err(CharAtError::ColumnOutOfBound);
        }
        let char_pos = line.first_char_pos().advance_by(coord.column.pred_count());
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
    #[error("Line does not exist")]
    LineOutOfBound,
}

impl<'a> LineAt<'a> for CompletedCharGrid {
    type Error = LineAtError;
    fn line_at(&'a self, ln_num: Ordinal) -> Result<Self::Line, LineAtError> {
        let (line, eol) = *self
            .line_list
            .get(ln_num.pred_count())
            .ok_or(LineAtError::LineOutOfBound)?;
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

/// An iterator that emits character cells from [`CompletedCharGrid`].
pub struct CompletedCharGridCharIter<'a>(slice::Iter<'a, CharCell>);

impl<'a> Iterator for CompletedCharGridCharIter<'a> {
    type Item = Result<CharCell, Infallible>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied()?.pipe(Ok).pipe(Some)
    }
}

impl<'a> IterChar<'a> for CompletedCharGrid {
    type Error = Infallible;
    type CharIter = Self::CharLoadIter;
    fn iter_char(&'a self) -> Self::CharIter {
        self.char_list().iter().pipe(CompletedCharGridCharIter)
    }
}

impl<'a> IterLoadChar<'a> for CompletedCharGrid {
    type Error = Infallible;
    type CharLoadIter = CompletedCharGridCharIter<'a>;
    fn iter_load_char(&'a mut self) -> Self::CharLoadIter {
        self.iter_char()
    }
}

/// An iterator that emits lines from a [`CompletedCharGrid`].
pub struct CompletedCharGridLineIter<'a> {
    iter: slice::Iter<'a, (TextSliceDef, EndOfLine)>,
    grid: &'a CompletedCharGrid,
}

impl<'a> Iterator for CompletedCharGridLineIter<'a> {
    type Item = Result<CharGridLine<'a, CompletedCharGrid>, Infallible>;
    fn next(&mut self) -> Option<Self::Item> {
        let (slice, eol) = *self.iter.next()?;
        CharGridLine::new(slice, eol, self.grid).pipe(Ok).pipe(Some)
    }
}

impl<'a> IterLine<'a> for CompletedCharGrid {
    type Error = Infallible;
    type LineIter = Self::LineLoadIter;
    fn iter_line(&'a self) -> Self::LineIter {
        CompletedCharGridLineIter {
            iter: self.line_list.iter(),
            grid: self,
        }
    }
}

impl<'a> IterLoadLine<'a> for CompletedCharGrid {
    type Line = CharGridLine<'a, Self>;
    type Error = Infallible;
    type LineLoadIter = CompletedCharGridLineIter<'a>;
    fn iter_load_line(&'a mut self) -> Self::LineLoadIter {
        self.iter_line()
    }
}
