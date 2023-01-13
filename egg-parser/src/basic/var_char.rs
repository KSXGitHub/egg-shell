use crate::{Parse, ParseResult, Response};
use derive_more::{Display, Error};
use egg_text::{CharAt, CharCell, CharOrEol, CharPos, CharPosOutOfBound, SliceFrom};
use pipe_trait::Pipe;

/// Parse a single character.
pub struct VarChar;

/// Failure type of [`VarChar`].
pub enum VarCharFailure {
    /// Input is empty.
    EmptyInput,
}

/// Error type of [`VarChar`].
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum VarCharFatalError<CharAtError, SliceFromError> {
    /// Error from [`CharAt`].
    CharAt(CharAtError),
    /// Error from [`SliceFrom`].
    SliceFrom(SliceFromError),
}

impl<Input, Stack> Parse<Input, Stack> for VarChar
where
    Input: CharAt<CharPos, Char = CharCell<CharOrEol>> + SliceFrom<CharPos, Slice = Input> + Copy,
    <Input as CharAt<CharPos>>::Error: TryInto<CharPosOutOfBound>,
{
    type Failure = VarCharFailure;
    type FatalError = VarCharFatalError<
        <<Input as CharAt<CharPos>>::Error as TryInto<CharPosOutOfBound>>::Error,
        <Input as SliceFrom<CharPos>>::Error,
    >;
    type Output = CharCell<CharOrEol>;

    fn parse(
        self,
        stack: Stack,
        input: Input,
    ) -> ParseResult<Input, Self::Output, Stack, Self::Failure, Self::FatalError> {
        let char = input
            .char_at(CharPos::from_pred_count(0))
            .map_err(TryInto::try_into);
        let output = match char {
            Ok(char) => char,
            Err(Ok(CharPosOutOfBound)) => return Response::failure_ok(VarCharFailure::EmptyInput),
            Err(Err(error)) => return error.pipe(VarCharFatalError::CharAt).pipe(Err),
        };
        let remaining = input
            .slice_from(CharPos::from_pred_count(1))
            .map_err(VarCharFatalError::SliceFrom)?;
        Response::builder()
            .with_stack(stack)
            .with_output(output)
            .with_remaining(remaining)
            .into_success()
            .into_ok()
    }
}
