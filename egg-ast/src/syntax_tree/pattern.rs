use crate::{Expression, Identifier, OptionalIdentifier, Span};
use never::Never;

pub type SinglePattern = Pattern<Never>;
pub type SinglePatternBody = PatternBody<Never>;
pub type SingleTuplePattern = TuplePattern<Never>;
pub type SingleDictPattern = DictPattern<Never>;
pub type SingleRenamePattern = RenamePattern<Never>;

#[derive(Debug)]
pub struct Pattern<Extra> {
    pub span: Span,
    pub body: PatternBody<Extra>,
}

#[derive(Debug)]
pub enum PatternBody<Extra> {
    Identifier(OptionalIdentifier),
    Tuple(TuplePattern<Extra>),
    Dict(DictPattern<Extra>),
    Extra(Extra),
}

#[derive(Debug)]
pub struct TuplePattern<Extra> {
    pub span: Span,
    pub head: Option<Expression>,
    pub body: Box<[Pattern<Extra>]>,
}

#[derive(Debug)]
pub struct DictPattern<Extra> {
    pub span: Span,
    pub head: Option<Expression>,
    pub body: Box<[RenamePattern<Extra>]>,
}

#[derive(Debug)]
pub struct RenamePattern<Extra> {
    pub key: Identifier,
    pub value: Pattern<Extra>,
}
