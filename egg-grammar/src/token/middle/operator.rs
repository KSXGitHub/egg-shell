/// Token a sequence of special characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorToken<Content>(pub Content);
