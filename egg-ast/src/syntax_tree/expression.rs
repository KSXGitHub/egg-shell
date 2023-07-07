mod field_access;
mod function_call;
mod macro_call;
mod member_access;

pub use field_access::*;
pub use function_call::*;
pub use macro_call::*;
pub use member_access::*;

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
    FieldAccess(FieldAccess),
    MemberAccess(MemberAccess),
}
