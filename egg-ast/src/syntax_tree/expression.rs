mod call;

pub use call::*;

use crate::{Literal, OptionalIdentifier, Span};

#[derive(Debug)]
pub struct Expression {
    pub span: Span,
    pub body: ExpressionBody,
}

#[derive(Debug)]
pub enum ExpressionBody {
    Literal(Literal),
    Identifier(OptionalIdentifier),
    FunctionCall(CallExpression),
}
