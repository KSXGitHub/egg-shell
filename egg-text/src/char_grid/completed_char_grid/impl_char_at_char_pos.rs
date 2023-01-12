use crate::{CharAt, CharCell, CharOrEol, CharPos, CharPosOutOfBound, CompletedCharGrid};
use derive_more::{Display, Error};
use std::convert::Infallible;

/// Error type of [`CharAt<CharPos>`] for [`CompletedCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum CharAtCharPosError {
    /// The grid doesn't have enough characters to match the requested index.
    #[display(fmt = "Character position does not exist")]
    OutOfBound,
}

impl TryFrom<CharAtCharPosError> for CharPosOutOfBound {
    type Error = Infallible;
    fn try_from(value: CharAtCharPosError) -> Result<CharPosOutOfBound, Infallible> {
        Ok(match value {
            CharAtCharPosError::OutOfBound => CharPosOutOfBound,
        })
    }
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

#[cfg(test)]
mod test;
