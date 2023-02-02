use super::{LineAtError, LoadCharError};
use crate::{CharAt, CharCell, CharOrEol, LazyCharGrid, LineAt, LnCol, LnColOutOfBound};
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use std::{cmp::Ordering, fmt::Debug};

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
    type Char = CharCell<CharOrEol>;
    type Error = CharAtLnColError<IterError>;

    fn char_at(self, coord: LnCol) -> Result<Self::Char, Self::Error> {
        let line = self.line_at(coord.line).map_err(|error| match error {
            LineAtError::LoadCharError(error) => CharAtLnColError::LoadCharError(error),
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
    }
}

#[cfg(test)]
mod test;
