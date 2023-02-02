use super::LineAtError;
use crate::{CharAt, CharCell, CharOrEol, CompletedCharGrid, LineAt, LnCol, LnColOutOfBound};
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use std::{cmp::Ordering, convert::Infallible};

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
    type Char = CharCell<CharOrEol>;
    type Error = CharAtLnColError;
    fn char_at(self, coord: LnCol) -> Result<Self::Char, CharAtLnColError> {
        let line = self.line_at(coord.line).map_err(|error| match error {
            LineAtError::OutOfBound => CharAtLnColError::LineOutOfBound,
        })?;
        match coord.column.pred_count().cmp(&line.char_count_wo_eol()) {
            Ordering::Greater => Err(CharAtLnColError::ColumnOutOfBound),
            Ordering::Equal => {
                Ok(CharCell {
                    coord,
                    offset_from_ln_start: line.size_wo_eol(),
                    offset_from_doc_start: line.end().offset(),
                    pos: line.start().index().advance_by(line.char_count_wo_eol()),
                    value: CharOrEol::EndOfLine(line.eol()),
                }) // TODO: replace this with CharGridLine::eol_cell
            }
            Ordering::Less => {
                let char_pos = line.start().index().advance_by(coord.column.pred_count());
                self.char_list()
                    .get(char_pos.pred_count())
                    .copied()
                    .expect("char_pos should be within the range of char_list")
                    .pipe(Self::Char::try_from)
                    .expect("resulting char should not be an EOL")
                    .pipe(Ok)
            }
        }
    }
}

#[cfg(test)]
mod test;
