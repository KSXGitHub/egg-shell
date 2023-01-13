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
