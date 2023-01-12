use crate::{char_grid::CharGridLine, CompletedCharGrid, EndOfLine, TextSliceDef, TryIterLine};
use std::{convert::Infallible, slice};

/// An iterator that emits lines from a [`CompletedCharGrid`].
pub struct LineIter<'a> {
    iter: slice::Iter<'a, (TextSliceDef, EndOfLine)>,
    grid: &'a CompletedCharGrid,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Result<CharGridLine<&'a CompletedCharGrid>, Infallible>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|&(slice, eol)| CharGridLine::new(slice, eol, self.grid))
            .map(Ok)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> TryIterLine for &'a CompletedCharGrid {
    type Line = CharGridLine<Self>;
    type Error = Infallible;
    type LineResultIter = LineIter<'a>;
    fn try_iter_line(self) -> Self::LineResultIter {
        LineIter {
            iter: self.line_list.iter(),
            grid: self,
        }
    }
}

#[cfg(test)]
mod test;
