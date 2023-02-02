use super::LoadCharError;
use crate::{
    char_grid::CharGridLine, CharPos, EndOfLine, LazyCharGrid, LineAt, LnCol, LnNum,
    LnNumOutOfBound, TextSliceSep,
};
use derive_more::{Display, Error};
use pipe_trait::Pipe;
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

    fn line_at(self, ln_end: LnNum) -> Result<Self::Line, LineAtError<IterError>> {
        // TODO: check for off-by-one error
        while self.data().loaded_eol_list.len() < ln_end.pred_count()
            && self.completion().is_incomplete()
        {
            self.load_line().map_err(LineAtError::LoadCharError)?;
        }

        let start = ln_end.try_retreat_by(1).and_then(|ln_start| {
            self.data()
                .loaded_eol_list
                .get(ln_start.pred_count())
                .map(|&(start, _)| start)
        });

        let default_last_line = || {
            start?; // start being None means that ln_end is way out-of-bound
            let offset = self.data().loaded_text.len();
            let index = self
                .data()
                .loaded_char_list
                .len()
                .pipe(CharPos::from_pred_count);
            let end = TextSliceSep { offset, index };
            Some((end, EndOfLine::EOF))
        };

        let (end, eol) = self
            .data()
            .loaded_eol_list
            .get(ln_end.pred_count())
            .copied()
            .or_else(default_last_line)
            .ok_or(LineAtError::OutOfBound)?;

        let get_first_line = |start: TextSliceSep| {
            let first_char_coord = self
                .data()
                .loaded_char_list
                .get(start.index().pred_count())?
                .coord();
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
