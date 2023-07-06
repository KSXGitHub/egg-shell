mod function_call;
mod macro_call;

pub use function_call::*;
pub use macro_call::*;

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
    FunctionCall(FunctionCallExpression),
    MacroCall(MacroCallExpression),
}
