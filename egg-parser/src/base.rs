mod response;

pub use response::*;

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
