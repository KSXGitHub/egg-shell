use super::CharGridLine;
use crate::{
    CharAt, CharCell, CharCoord, CharCount, EndOfLine, IterChar, IterLoadChar, LoadCharAt,
    TextSliceDef,
};
use getset::{CopyGetters, Getters};
use pipe_trait::Pipe;
use std::{convert::Infallible, iter, slice};
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

impl CharAt for CompletedCharGrid {
    type Error = CharAtError;
    fn char_at(&self, coord: CharCoord) -> Result<CharCell, CharAtError> {
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

impl LoadCharAt for CompletedCharGrid {
    type Error = CharAtError;
    fn load_char_at(&mut self, coord: CharCoord) -> Result<CharCell, CharAtError> {
        self.char_at(coord)
    }
}

impl CharCount for CompletedCharGrid {
    fn char_count(&self) -> usize {
        self.char_list().len()
    }
}

impl<'a> IterChar<'a> for CompletedCharGrid {
    type Error = Infallible;
    type CharIter = Self::CharLoadIter;
    fn iter_char(&'a self) -> Self::CharIter {
        self.char_list().iter().copied().map(Ok)
    }
}

impl<'a> IterLoadChar<'a> for CompletedCharGrid {
    type Error = Infallible;
    type CharLoadIter = iter::Map<
        iter::Copied<slice::Iter<'a, CharCell>>,
        fn(CharCell) -> Result<CharCell, Infallible>,
    >;
    fn iter_load_char(&'a mut self) -> Self::CharLoadIter {
        self.iter_char()
    }
}
