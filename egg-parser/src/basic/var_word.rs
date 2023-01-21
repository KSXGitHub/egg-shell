mod end_of_word;

pub use end_of_word::*;

use crate::{Parse, ParseResult, Response};
use derive_more::{Constructor, Display, Error};
use egg_text::{
    char_grid::PartiallyClonedCharGrid, CharAt, CharCell, CharOrEol, CharPos, CharPosOutOfBound,
    SliceFrom,
};
use pipe_trait::Pipe;
use std::convert::Infallible;

/// Parse a word.
#[derive(Debug, Clone, Copy, Constructor)]
pub struct VarWord<End: IsEndOfWord> {
    end: End,
}

/// Failure type of [`VarWord`].
pub type VarWordFailure = Infallible;

/// Error type of [`VarWord`].
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum VarWordFatalError<CharAtError, SliceFromError> {
    /// Error from [`CharAt`].
    CharAt(CharAtError),
    /// Error from [`SliceFrom`].
    SliceFrom(SliceFromError),
}

impl<Input, Stack, End> Parse<Input, Stack> for VarWord<End>
where
    Input: CharAt<CharPos, Char = CharCell<CharOrEol>> + SliceFrom<CharPos, Slice = Input> + Copy,
    <Input as CharAt<CharPos>>::Error: TryInto<CharPosOutOfBound>,
    End: IsEndOfWord,
{
    type Failure = VarWordFailure;
    type FatalError = VarWordFatalError<
        <<Input as CharAt<CharPos>>::Error as TryInto<CharPosOutOfBound>>::Error,
        <Input as SliceFrom<CharPos>>::Error,
    >;
    type Output = PartiallyClonedCharGrid;

    fn parse(
        self,
        stack: Stack,
        input: Input,
    ) -> ParseResult<Input, Self::Output, Stack, Self::Failure, Self::FatalError> {
        let mut output = PartiallyClonedCharGrid::default();

        loop {
            let pos = CharPos::from_pred_count(output.char_count());
            let char = input.char_at(pos).map_err(TryInto::try_into);
            let char = match char {
                Ok(char) => char,
                Err(Ok(CharPosOutOfBound)) => break,
                Err(Err(error)) => return error.pipe(VarWordFatalError::CharAt).pipe(Err),
            };
            if self.end.is_end_of_word(*char.value()) {
                break;
            }
            output.push(char);
        }

        let pos = CharPos::from_pred_count(output.char_count());
        let remaining = input
            .slice_from(pos)
            .map_err(VarWordFatalError::SliceFrom)?;
        Response::builder()
            .with_stack(stack)
            .with_output(output)
            .with_remaining(remaining)
            .into_success()
            .into_ok()
    }
}
