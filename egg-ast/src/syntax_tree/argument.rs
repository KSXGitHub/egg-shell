use crate::{Expression, Identifier, Span};

#[derive(Debug)]
pub struct ArgumentList {
    pub span: Span,
    pub body: Box<[Argument]>,
}

#[derive(Debug)]
pub struct Argument {
    pub span: Span,
    pub body: ArgumentBody,
}

#[derive(Debug)]
pub enum ArgumentBody {
    Named(NamedArgument),
    Positional(Expression),
}

#[derive(Debug)]
pub struct NamedArgument {
    pub span: Span,
    pub key: Identifier,
    pub value: Expression,
}
