use super::LoadCharError;
use crate::{char_grid::CharGridLine, LazyCharGrid, LineAt, LnNum, LnNumOutOfBound};
use derive_more::{Display, Error};
use std::fmt::Debug;

/// Error type of [`LineAt`] for [`LazyCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum LineAtError<IterError> {
    /// An error occurred while loading a character.
    LoadCharError(LoadCharError<IterError>),
    /// The source iterator doesn't have enough lines to match the requested index.
    #[display(fmt = "Line does not exist")]
    OutOfBound,
}

impl<IterError> TryFrom<LineAtError<IterError>> for LnNumOutOfBound {
    type Error = LoadCharError<IterError>;
    fn try_from(value: LineAtError<IterError>) -> Result<Self, Self::Error> {
        match value {
            LineAtError::LoadCharError(error) => Err(error),
            LineAtError::OutOfBound => Ok(LnNumOutOfBound),
        }
    }
}

impl<'a, IterError, CharIter> LineAt<LnNum> for &'a LazyCharGrid<CharIter>
where
    CharIter: Iterator<Item = Result<char, IterError>> + 'a,
{
    type Line = CharGridLine<Self>;
    type Error = LineAtError<IterError>;
    fn line_at(self, ln_num: LnNum) -> Result<Self::Line, LineAtError<IterError>> {
        while self.data().loaded_line_list.len() <= ln_num.pred_count()
            && self.completion().is_incomplete()
        {
            self.load_line().map_err(LineAtError::LoadCharError)?;
        }
        if let Some((slice, eol)) = self.data().loaded_line_list.get(ln_num.pred_count()) {
            return Ok(CharGridLine::new(*slice, *eol, self));
        }
        Err(LineAtError::OutOfBound)
    }
}

#[cfg(test)]
mod test;
