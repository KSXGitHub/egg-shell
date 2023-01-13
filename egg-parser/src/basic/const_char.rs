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
        let output = input
            .char_at(CharPos::from_pred_count(0))
            .map_err(ConstCharFatalError::CharAt)?;
        if output.value() != &CharOrEol::Char(self.0) {
            return output
                .pipe(ConstCharFailure::Mismatch)
                .pipe(Response::Failure)
                .pipe(Ok);
        }
        let remaining = input
            .slice_from(CharPos::from_pred_count(1))
            .map_err(ConstCharFatalError::SliceFrom)?;
        Response::builder()
            .with_stack(stack)
            .with_output(output)
            .with_remaining(remaining)
            .into_success()
            .into_ok()
    }
}
