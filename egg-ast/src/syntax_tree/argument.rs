use crate::{Expression, Identifier, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentList {
    pub span: Span,
    pub body: Vec<Argument>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Argument {
    pub span: Span,
    pub body: ArgumentBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentBody {
    Named(NamedArgument),
    Positional(Expression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedArgument {
    pub span: Span,
    pub key: Identifier,
    pub value: Expression,
}
