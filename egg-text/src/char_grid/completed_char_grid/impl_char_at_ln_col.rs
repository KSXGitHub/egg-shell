use super::LineAtError;
use crate::{CharAt, CharCell, CompletedCharGrid, LineAt, LnCol, LnColOutOfBound};
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use std::convert::Infallible;

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

impl TryFrom<CharAtLnColError> for LnColOutOfBound {
    type Error = Infallible;
    fn try_from(value: CharAtLnColError) -> Result<LnColOutOfBound, Infallible> {
        Ok(match value {
            CharAtLnColError::LineOutOfBound => LnColOutOfBound::LineOutOfBound,
            CharAtLnColError::ColumnOutOfBound => LnColOutOfBound::ColumnOutOfBound,
        })
    }
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

#[cfg(test)]
mod test;
