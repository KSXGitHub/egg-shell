use crate::{
    char_grid::CharGridLine, CharPos, CompletedCharGrid, EndOfLine, LineAt, LnCol, LnNum,
    TextSliceSep,
};
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

    fn line_at(self, ln_end: LnNum) -> Result<Self::Line, LineAtError> {
        let start = ln_end.try_retreat_by(1).and_then(|ln_start| {
            self.eol_list
                .get(ln_start.pred_count())
                .map(|&(start, _)| start)
        });

        let default_last_line = || {
            start?; // start being None means that ln_end is way out-of-bound
            let offset = self.text.len();
            let index = CharPos::from_pred_count(self.char_count);
            let end = TextSliceSep { offset, index };
            Some((end, EndOfLine::EOF))
        };

        let (end, eol) = self
            .eol_list
            .get(ln_end.pred_count())
            .copied()
            .or_else(default_last_line)
            .ok_or(LineAtError::OutOfBound)?;

        let get_first_line = |start: TextSliceSep| {
            let first_char_coord = self.char_list.get(start.index().pred_count())?.coord;
            Some((first_char_coord, start))
        };

        let default_first_line = || (LnCol::from_pred_counts(0, 0), TextSliceSep::ZERO);

        let (first_char_coord, start) = start
            .and_then(get_first_line)
            .unwrap_or_else(default_first_line);

        let grid = self;

        Ok(CharGridLine {
            first_char_coord,
            start,
            end,
            eol,
            grid,
        })
    }
}

#[cfg(test)]
mod test;
