use super::LoadCharError;
use crate::{CharAt, CharCell, CharOrEol, CharPos, CharPosOutOfBound, LazyCharGrid};
use derive_more::{Display, Error};
use std::fmt::Debug;

/// Error type of [`CharAt<CharPos>`] for [`LazyCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum CharAtCharPosError<IterError> {
    /// An error occurred while loading a character.
    LoadCharError(LoadCharError<IterError>),
    /// The grid doesn't have enough characters to match the requested index.
    #[display(fmt = "Character position does not exist")]
    OutOfBound,
}

impl<IterError> TryFrom<CharAtCharPosError<IterError>> for CharPosOutOfBound {
    type Error = LoadCharError<IterError>;
    fn try_from(value: CharAtCharPosError<IterError>) -> Result<Self, Self::Error> {
        match value {
            CharAtCharPosError::LoadCharError(error) => Err(error),
            CharAtCharPosError::OutOfBound => Ok(CharPosOutOfBound),
        }
    }
}

impl<'a, IterError, CharIter> CharAt<CharPos> for &'a LazyCharGrid<CharIter>
where
    CharIter: Iterator<Item = Result<char, IterError>>,
{
    type Char = CharCell<CharOrEol>;
    type Error = CharAtCharPosError<IterError>;

    fn char_at(self, pos: CharPos) -> Result<Self::Char, Self::Error> {
        while self.completion().is_incomplete() && pos.pred_count() >= self.loaded_char_count() {
            self.load_char()
                .map_err(CharAtCharPosError::LoadCharError)?;
        }
        self.data()
            .loaded_char_list()
            .get(pos.pred_count())
            .copied()
            .ok_or(CharAtCharPosError::OutOfBound)
    }
}
