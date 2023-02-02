use super::LineAtError;
use crate::{char_grid::CharGridLine, CompletedCharGrid, LineAt, LnNum, TryIterLine};
use std::convert::Infallible;

/// An iterator that emits lines from a [`CompletedCharGrid`].
pub struct LineIter<'a> {
    index: LnNum,
    grid: &'a CompletedCharGrid,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Result<CharGridLine<&'a CompletedCharGrid>, Infallible>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index = index.advance_by(1);
        match self.grid.line_at(index) {
            Err(LineAtError::OutOfBound) => None,
            Ok(line) => Some(Ok(line)),
        }
    }
}

impl<'a> TryIterLine for &'a CompletedCharGrid {
    type Line = CharGridLine<Self>;
    type Error = Infallible;
    type LineResultIter = LineIter<'a>;
    fn try_iter_line(self) -> Self::LineResultIter {
        LineIter {
            index: LnNum::from_pred_count(0),
            grid: self,
        }
    }
}

#[cfg(test)]
mod test;
