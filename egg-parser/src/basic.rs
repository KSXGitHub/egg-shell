use crate::{Parse, ParseResult, Response};
use egg_text::{CharAt, CharCell, CharOrEol, CharPos, SliceFrom};

/// Parse a character.
pub struct Char(pub char);

/// Error type of [`Char`].
pub enum CharError<CharAtError, SliceFromError> {
    /// Not the right character.
    Mismatch(CharCell<CharOrEol>),
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
    type Error = CharError<<Input as CharAt<CharPos>>::Error, <Input as SliceFrom<CharPos>>::Error>;
    type Output = CharCell<CharOrEol>;

    fn parse(
        self,
        stack: Self::Stack,
        input: Input,
    ) -> ParseResult<Input, Self::Output, Self::Stack, Self::Error> {
        let output = input
            .char_at(CharPos::from_pred_count(0))
            .map_err(CharError::CharAt)?;
        if output.value() != &CharOrEol::Char(self.0) {
            return Err(CharError::Mismatch(output));
        }
        let remaining = input
            .slice_from(CharPos::from_pred_count(1))
            .map_err(CharError::SliceFrom)?;
        Ok(Response {
            stack,
            output,
            remaining,
        })
    }
}
