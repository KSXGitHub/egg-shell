/// Success value of [`Parse::parse`](super::Parse::parse).
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

    /// Convert into a [`Response::Success`].
    pub const fn into_success<Failure>(self) -> Response<Input, Output, Stack, Failure> {
        Response::Success(self)
    }

    /// Replace [`ResponseValue::remaining`].
    pub fn with_remaining<NewInput>(
        self,
        remaining: NewInput,
    ) -> ResponseValue<NewInput, Output, Stack> {
        let ResponseValue { stack, output, .. } = self;
        ResponseValue {
            stack,
            output,
            remaining,
        }
    }

    /// Replace [`ResponseValue::stack`].
    pub fn with_stack<NewStack>(self, stack: NewStack) -> ResponseValue<Input, Output, NewStack> {
        let ResponseValue {
            output, remaining, ..
        } = self;
        ResponseValue {
            stack,
            output,
            remaining,
        }
    }

    /// Replace [`ResponseValue::output`].
    pub fn with_output<NewOutput>(
        self,
        output: NewOutput,
    ) -> ResponseValue<Input, NewOutput, Stack> {
        let ResponseValue {
            stack, remaining, ..
        } = self;
        ResponseValue {
            stack,
            output,
            remaining,
        }
    }
}

impl ResponseValue<(), (), ()> {
    /// Start a builder pattern.
    pub const fn builder() -> Self {
        ResponseValue {
            stack: (),
            output: (),
            remaining: (),
        }
    }
}

/// Response of [`Parse::parse`](super::Parse::parse).
#[derive(Debug, Clone, Copy)]
pub enum Response<Input, Output, Stack, Failure> {
    /// Value when parsing succeeds.
    Success(ResponseValue<Input, Output, Stack>),
    /// Error when parsing fails.
    Failure(Failure),
}

impl<Input, Output, Stack, Failure> Response<Input, Output, Stack, Failure> {
    /// Convert into a [`Result::Ok`].
    pub const fn into_ok<FatalError>(self) -> Result<Self, FatalError> {
        Ok(self)
    }
}

impl Response<(), (), (), ()> {
    /// Start a builder pattern.
    pub const fn builder() -> ResponseValue<(), (), ()> {
        ResponseValue::builder()
    }
}
