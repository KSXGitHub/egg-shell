use super::{LineAtError, LoadCharError};
use crate::{char_grid::CharGridLine, LazyCharGrid, LineAt, LnNum, TryIterLine};

/// An iterator that emits instances of [`CharGridLine`] from [`LazyCharGrid`].
pub struct LineIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    index: LnNum,
    grid: &'a LazyCharGrid<SrcIter>,
}

impl<'a, SrcIterError, SrcIter> Iterator for LineIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Item = Result<CharGridLine<&'a LazyCharGrid<SrcIter>>, LoadCharError<SrcIterError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index = index.advance_by(1);
        let line = self.grid.line_at(index);
        match line {
            Err(LineAtError::LoadCharError(error)) => Some(Err(error)),
            Err(LineAtError::OutOfBound) => None,
            Ok(line) => Some(Ok(line)),
        }
    }
}

impl<'a, SrcIterError, SrcIter> TryIterLine for &'a LazyCharGrid<SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Line = CharGridLine<Self>;
    type Error = LoadCharError<SrcIterError>;
    type LineResultIter = LineIter<'a, SrcIterError, SrcIter>;

    fn try_iter_line(self) -> Self::LineResultIter {
        LineIter {
            index: LnNum::from_pred_count(0),
            grid: self,
        }
    }
}

#[cfg(test)]
mod test;
