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
        // if coord.column.pred_count() >= line.slice().char_count() {
        //     return Err(CharAtLnColError::ColumnOutOfBound);
        // }
        match coord.column.pred_count().cmp(&line.slice().char_count()) {
            Ordering::Greater => Err(CharAtLnColError::ColumnOutOfBound),
            Ordering::Equal => {
                let ln_size = line.text_without_eol().len();
                let slice = line.slice();
                Ok(CharCell {
                    coord,
                    offset_from_ln_start: ln_size,
                    offset_from_doc_start: slice.offset() + ln_size,
                    pos: slice.first_char_pos().advance_by(slice.char_count()),
                    value: CharOrEol::EndOfLine(line.eol()),
                })
            }
            Ordering::Less => {
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
    }
}

#[cfg(test)]
mod test;
