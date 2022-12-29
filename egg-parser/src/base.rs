/// Success value of [`Parse::parse`].
#[derive(Debug, Clone, Copy)]
pub struct Response<Input, Output> {
    pub output: Output,
    pub remaining: Input,
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
