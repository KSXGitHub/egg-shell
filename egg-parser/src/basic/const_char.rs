use super::{VarChar, VarCharFatalError};
use crate::{Parse, ParseResult, Response};
use derive_more::{Display, Error};
use egg_text::{CharAt, CharCell, CharOrEol, CharPos, SliceFrom};
use pipe_trait::Pipe;

/// Parse a character constant.
#[derive(Debug, Clone, Copy)]
pub struct ConstChar(pub char);

/// Failure type of [`ConstChar`].
#[derive(Debug, Clone, Copy)]
pub enum ConstCharFailure {
    /// Not the right character.
    Mismatch(CharCell<CharOrEol>),
}

/// Error type of [`ConstChar`].
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum ConstCharFatalError<CharAtError, SliceFromError> {
    /// Error from [`CharAt`].
    CharAt(CharAtError),
    /// Error from [`SliceFrom`].
    SliceFrom(SliceFromError),
}

impl<Input, Stack> Parse<Input, Stack> for ConstChar
where
    Input: CharAt<CharPos, Char = CharCell<CharOrEol>> + SliceFrom<CharPos, Slice = Input> + Copy,
{
    type Failure = ConstCharFailure;
    type FatalError = ConstCharFatalError<
        <Input as CharAt<CharPos>>::Error,
        <Input as SliceFrom<CharPos>>::Error,
    >;
    type Output = CharCell<CharOrEol>;

    fn parse(
        self,
        stack: Stack,
        input: Input,
    ) -> ParseResult<Input, Self::Output, Stack, Self::Failure, Self::FatalError> {
        let response = VarChar.parse(stack, input).map_err(|error| match error {
            VarCharFatalError::CharAt(error) => ConstCharFatalError::CharAt(error),
            VarCharFatalError::SliceFrom(error) => ConstCharFatalError::SliceFrom(error),
        })?;
        let response = match response {
            Response::Success(output) => output,
            Response::Failure(error) => match error {},
        };
        if response.output.value() != &CharOrEol::Char(self.0) {
            response
                .output
                .pipe(ConstCharFailure::Mismatch)
                .pipe(Response::Failure)
                .pipe(Ok)
        } else {
            response.into_success().into_ok()
        }
    }
}
