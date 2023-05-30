use crate::{OptionalIdentifier, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SinglePattern {
    pub span: Span,
    pub body: SinglePatternBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SinglePatternBody {
    Identifier(OptionalIdentifier),
}
