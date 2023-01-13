/// Success value of [`Parse::parse`].
#[derive(Debug, Clone, Copy)]
pub struct ResponseValue<Input, Output, Stack> {
    /// Stack from parent parsers.
    pub stack: Stack,
    /// Output emitted by the parser.
    pub output: Output,
    /// Remaining unparsed input.
    pub remaining: Input,
}

impl<Input, Output, Stack> ResponseValue<Input, Output, Stack> {
    /// Create a [`ResponseValue`] from a tuple of [stack](ResponseValue::stack) from parent parsers,
    /// [output](ResponseValue::output) of current parser, and [remaining unparsed input](ResponseValue::remaining).
    pub const fn from_tuple(stack: Stack, output: Output, remaining: Input) -> Self {
        ResponseValue {
            stack,
            output,
            remaining,
        }
    }

    /// Convert the [`ResponseValue`] into a tuple of [stack](ResponseValue::stack) from parent parsers,
    /// [output](ResponseValue::output) of current parser, and [remaining unparsed input](ResponseValue::remaining).
    pub fn into_tuple(self) -> (Stack, Output, Input) {
        (self.stack, self.output, self.remaining)
    }
}

/// Response of [`Parse::parse`].
#[derive(Debug, Clone, Copy)]
pub enum Response<Input, Output, Stack, Failure> {
    /// Value when parsing succeeds.
    Success(ResponseValue<Input, Output, Stack>),
    /// Error when parsing fails.
    Failure(Failure),
}

/// Return type of [`Parse::parse`].
pub type ParseResult<Input, Output, Stack, Failure, FatalError> =
    Result<Response<Input, Output, Stack, Failure>, FatalError>;

/// Parse an input.
pub trait Parse<Input> {
    /// Stack to pass between parsers.
    type Stack;
    /// Parsing result.
    ///
    /// The parser should return this value.
    type Output;
    /// Parsing failure.
    ///
    /// Try again with other parser.
    type Failure;
    /// Irrecoverable error.
    ///
    /// The parser should terminate with this error if encounter.
    type FatalError;
    /// Parse an input.
    fn parse(
        self,
        parent_stack: Self::Stack,
        input: Input,
    ) -> ParseResult<Input, Self::Output, Self::Stack, Self::Failure, Self::FatalError>;
}
