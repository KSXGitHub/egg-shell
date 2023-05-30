use crate::{OptionalIdentifier, Span};
use never::Never;

pub type SinglePattern = Pattern<Never>;
pub type SinglePatternBody = PatternBody<Never>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern<Extra> {
    pub span: Span,
    pub body: PatternBody<Extra>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternBody<Extra> {
    Identifier(OptionalIdentifier),
    Extra(Extra),
}
