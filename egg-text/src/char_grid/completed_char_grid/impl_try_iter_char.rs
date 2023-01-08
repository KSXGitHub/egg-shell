use crate::{CharCell, CharOrEol, ColNum, CompletedCharGrid, LineAt, LnCol, LnNum, TryIterChar};
use pipe_trait::Pipe;
use std::{cmp::Ordering, convert::Infallible};

/// An iterator that emits character cells from [`CompletedCharGrid`].
pub struct CharIter<'a> {
    ln_index: LnNum,
    col_index: ColNum,
    grid: &'a CompletedCharGrid,
}

impl<'a> Iterator for CharIter<'a> {
    type Item = Result<CharCell<CharOrEol>, Infallible>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.grid.line_at(self.ln_index).ok()?;
        match self.col_index.pred_count().cmp(&line.slice().char_count()) {
            Ordering::Greater => panic!("Column index should never be greater than line count"),
            Ordering::Equal => {
                let coord = LnCol {
                    line: self.ln_index,
                    column: self.col_index,
                };
                let col_count = coord.column.pred_count(); // number of columns from line start
                let pos = line.slice().first_char_pos().advance_by(col_count);
                self.ln_index = self.ln_index.advance_by(1);
                self.col_index = ColNum::from_pred_count(0);
                let offset_from_ln_start = line.slice().size();
                let offset_from_doc_start = line.slice().offset() + line.slice().size();
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
                let char_pos = line
                    .slice()
                    .first_char_pos()
                    .advance_by(self.col_index.pred_count());
                self.col_index = self.col_index.advance_by(1);
                self.grid
                    .char_list()
                    .get(char_pos.pred_count())
                    .copied()?
                    .map(CharOrEol::from)
                    .pipe(Ok)
                    .pipe(Some)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let max = self.grid.text.len();
        let min = max / std::mem::size_of::<char>();
        (min, Some(max))
    }
}

impl<'a> TryIterChar for &'a CompletedCharGrid {
    type Char = CharCell<CharOrEol>;
    type Error = Infallible;
    type CharResultIter = CharIter<'a>;
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
