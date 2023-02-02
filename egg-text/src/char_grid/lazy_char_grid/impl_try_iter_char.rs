use super::{LineAtError, LoadCharError};
use crate::{CharCell, CharOrEol, ColNum, LazyCharGrid, LineAt, LnCol, LnNum, TryIterChar};
use pipe_trait::Pipe;
use std::cmp::Ordering;

/// An iterator that emits instances of [`CharCell`] from [`LazyCharGrid`].
pub struct CharIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    ln_index: LnNum,
    col_index: ColNum,
    grid: &'a LazyCharGrid<SrcIter>,
}

impl<'a, SrcIterError, SrcIter> Iterator for CharIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Item = Result<CharCell<CharOrEol>, LoadCharError<SrcIterError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = match self.grid.line_at(self.ln_index) {
            Err(LineAtError::LoadCharError(error)) => return Some(Err(error)),
            Err(LineAtError::OutOfBound) => return None,
            Ok(line) => line,
        };
        match self.col_index.pred_count().cmp(&line.char_count_wo_eol()) {
            Ordering::Greater => panic!("Column index should never be greater than line count"),
            Ordering::Equal => {
                let coord = LnCol {
                    line: self.ln_index,
                    column: self.col_index,
                };
                let col_count = coord.column.pred_count(); // number of columns from line start
                let pos = line.start().index().advance_by(col_count);
                self.ln_index = self.ln_index.advance_by(1);
                self.col_index = ColNum::from_pred_count(0);
                let offset_from_ln_start = line.size_wo_eol();
                let offset_from_doc_start = line.end().offset();
                let value = CharOrEol::EndOfLine(line.eol());
                let char_cell = CharCell {
                    coord,
                    pos,
                    offset_from_ln_start,
                    offset_from_doc_start,
                    value,
                };
                Some(Ok(char_cell))
            }
            Ordering::Less => {
                let char_pos = line.start().index().advance_by(self.col_index.pred_count());
                self.col_index = self.col_index.advance_by(1);
                self.grid
                    .data()
                    .loaded_char_list()
                    .get(char_pos.pred_count())
                    .copied()?
                    .map(CharOrEol::from)
                    .pipe(Ok)
                    .pipe(Some)
            }
        }
    }
}

impl<'a, SrcIterError, SrcIter> TryIterChar for &'a LazyCharGrid<SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Char = CharCell<CharOrEol>;
    type Error = LoadCharError<SrcIterError>;
    type CharResultIter = CharIter<'a, SrcIterError, SrcIter>;

    fn try_iter_char(self) -> Self::CharResultIter {
        // Q: Why not just iterate over existing self.data.char_list?
        // A: data.char_list is very expensive in term of memory, it is to be removed in the future.
        CharIter {
            ln_index: LnNum::from_pred_count(0),
            col_index: ColNum::from_pred_count(0),
            grid: self,
        }
    }
}

#[cfg(test)]
mod test;
