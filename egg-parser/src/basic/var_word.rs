use crate::{Parse, ParseResult, Response};
use derive_more::{Constructor, Display, Error};
use egg_text::{CharAt, CharCell, CharOrEol, CharPos, CharPosOutOfBound, SliceFrom};
use pipe_trait::Pipe;
use std::convert::Infallible;

/// Check if a character is the end of a word.
pub trait IsEndOfWord: Copy {
    /// Check if a character is the end of a word.
    fn is_end_of_word(self, char: CharOrEol) -> bool;
}

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
    type Output = Vec<CharCell<CharOrEol>>; // TODO: Create a grid type to replace this

    fn parse(
        self,
        stack: Stack,
        input: Input,
    ) -> ParseResult<Input, Self::Output, Stack, Self::Failure, Self::FatalError> {
        let mut output: Self::Output = Vec::new();
        let mut pos = CharPos::from_pred_count(0);

        loop {
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
            pos = pos.advance_by(1);
        }

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
