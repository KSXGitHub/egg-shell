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
    Tuple(TuplePattern<Extra>),
    Extra(Extra),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuplePattern<Extra> {
    pub span: Span,
    pub head: Option<OptionalIdentifier>,
    pub body: Vec<Pattern<Extra>>,
}
