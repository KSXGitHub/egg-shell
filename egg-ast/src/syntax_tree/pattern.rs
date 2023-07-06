use crate::{Expression, Identifier, MutabilityModifier, OptionalIdentifier, Span};

#[derive(Debug)]
pub struct Pattern {
    pub span: Span,
    pub body: PatternBody,
}

#[derive(Debug)]
pub enum PatternBody {
    Identifier(IdentifierPattern),
    Tuple(TuplePattern),
    Dict(DictPattern),
}

#[derive(Debug)]
pub struct IdentifierPattern {
    pub mutability: Option<MutabilityModifier>,
    pub identifier: OptionalIdentifier,
}

#[derive(Debug)]
pub struct TuplePattern {
    pub span: Span,
    pub head: Option<Expression>,
    pub body: Box<[Pattern]>,
}

#[derive(Debug)]
pub struct DictPattern {
    pub span: Span,
    pub head: Option<Expression>,
    pub body: Box<[RenamePattern]>,
}

#[derive(Debug)]
pub struct RenamePattern {
    pub key: Identifier,
    pub value: Pattern,
}
