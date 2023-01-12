use crate::{char_grid::CharGridLine, CompletedCharGrid, LineAt, LnNum};
use derive_more::{Display, Error};

/// Error type of [`LineAt`] for [`CompletedCharGrid`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum LineAtError {
    /// The line doesn't have enough characters to match the requested column index.
    #[display(fmt = "Column does not exist")]
    OutOfBound,
}

impl<'a> LineAt<LnNum> for &'a CompletedCharGrid {
    type Line = CharGridLine<Self>;
    type Error = LineAtError;
    fn line_at(self, ln_num: LnNum) -> Result<Self::Line, LineAtError> {
        self.line_list
            .get(ln_num.pred_count())
            .map(|&(slice, eol)| CharGridLine::new(slice, eol, self))
            .ok_or(LineAtError::OutOfBound)
    }
}

#[cfg(test)]
mod test;
