use crate::{Parse, ParseResult, Response, ResponseValue};
use egg_text::{CharAt, CharCell, CharOrEol, CharPos, SliceFrom};
use pipe_trait::Pipe;

/// Parse a character.
pub struct Char(pub char);

/// Failure type of [`Char`].
pub enum CharFailure {
    /// Not the right character.
    Mismatch(CharCell<CharOrEol>),
}

/// Error type of [`Char`].
pub enum CharFatalError<CharAtError, SliceFromError> {
    /// Error from [`CharAt`].
    CharAt(CharAtError),
    /// Error from [`SliceFrom`].
    SliceFrom(SliceFromError),
}

impl<Input> Parse<Input> for Char
where
    Input: CharAt<CharPos, Char = CharCell<CharOrEol>> + SliceFrom<CharPos, Slice = Input> + Copy,
{
    type Stack = ();
    type Failure = CharFailure;
    type FatalError =
        CharFatalError<<Input as CharAt<CharPos>>::Error, <Input as SliceFrom<CharPos>>::Error>;
    type Output = CharCell<CharOrEol>;

    fn parse(
        self,
        stack: Self::Stack,
        input: Input,
    ) -> ParseResult<Input, Self::Output, Self::Stack, Self::Failure, Self::FatalError> {
        let output = input
            .char_at(CharPos::from_pred_count(0))
            .map_err(CharFatalError::CharAt)?;
        if output.value() != &CharOrEol::Char(self.0) {
            return output
                .pipe(CharFailure::Mismatch)
                .pipe(Response::Failure)
                .pipe(Ok);
        }
        let remaining = input
            .slice_from(CharPos::from_pred_count(1))
            .map_err(CharFatalError::SliceFrom)?;
        ResponseValue::builder()
            .with_stack(stack)
            .with_output(output)
            .with_remaining(remaining)
            .into_success()
            .into_ok()
    }
}
