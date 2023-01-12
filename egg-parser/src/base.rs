/// Success value of [`Parse::parse`].
#[derive(Debug, Clone, Copy)]
pub struct Response<Input, Output, Stack> {
    /// Stack from parent parsers.
    pub stack: Stack,
    /// Output emitted by the parser.
    pub output: Output,
    /// Remaining unparsed input.
    pub remaining: Input,
}

impl<Input, Output, Stack> Response<Input, Output, Stack> {
    /// Create a [`Response`] from a tuple of [stack](Response::stack) from parent parsers,
    /// [output](Response::output) of current parser, and [remaining unparsed input](Response::remaining).
    pub const fn from_tuple(stack: Stack, output: Output, remaining: Input) -> Self {
        Response {
            stack,
            output,
            remaining,
        }
    }

    /// Convert the [`Response`] into a tuple of [stack](Response::stack) from parent parsers,
    /// [output](Response::output) of current parser, and [remaining unparsed input](Response::remaining).
    pub fn into_tuple(self) -> (Stack, Output, Input) {
        (self.stack, self.output, self.remaining)
    }
}

/// Return type of [`Parse::parse`].
pub type ParseResult<Input, Output, Stack, Error> = Result<Response<Input, Output, Stack>, Error>;

/// Parse an input.
pub trait Parse<Input> {
    /// Stack to pass between parsers.
    type Stack;
    /// Parsing result.
    type Output;
    /// Parsing error.
    type Error;
    /// Parse an input.
    fn parse(
        parent_stack: Self::Stack,
        input: Input,
    ) -> ParseResult<Input, Self::Output, Self::Stack, Self::Error>;
}