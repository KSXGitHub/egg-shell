/// Success value of [`Parse::parse`].
#[derive(Debug, Clone, Copy)]
pub struct Response<Input, Output> {
    /// Output emitted by the parser.
    pub output: Output,
    /// Remaining unparsed input.
    pub remaining: Input,
}

impl<Input, Output> Response<Input, Output> {
    /// Create a [`Response`] from a tuple of output and remaining input.
    pub const fn from_tuple(output: Output, remaining: Input) -> Self {
        Response { output, remaining }
    }

    /// Convert the [`Response`] into a tuple of output and remaining input.
    pub fn into_tuple(self) -> (Output, Input) {
        (self.output, self.remaining)
    }
}

/// Return type of [`Parse::parse`].
pub type ParseResult<Input, Output, Error> = Result<Response<Input, Output>, Error>;

/// Parse an input.
pub trait Parse<Input> {
    /// Parsing result.
    type Output;
    /// Parsing error.
    type Error;
    /// Parse an input.
    fn parse(input: Input) -> ParseResult<Input, Self::Output, Self::Error>;
}
