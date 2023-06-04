use crate::{Identifier, OptionalIdentifier, Span};
use never::Never;

pub type SinglePattern = Pattern<Never>;
pub type SinglePatternBody = PatternBody<Never>;
pub type SingleTuplePattern = TuplePattern<Never>;
pub type SingleDictPattern = DictPattern<Never>;
pub type SingleRenamePattern = RenamePattern<Never>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern<Extra> {
    pub span: Span,
    pub body: PatternBody<Extra>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternBody<Extra> {
    Identifier(OptionalIdentifier),
    Tuple(TuplePattern<Extra>),
    Dict(DictPattern<Extra>),
    Extra(Extra),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuplePattern<Extra> {
    pub span: Span,
    pub head: Option<OptionalIdentifier>,
    pub body: Vec<Pattern<Extra>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictPattern<Extra> {
    pub span: Span,
    pub head: Option<OptionalIdentifier>,
    pub body: Vec<RenamePattern<Extra>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenamePattern<Extra> {
    pub key: Identifier,
    pub value: Pattern<Extra>,
}
