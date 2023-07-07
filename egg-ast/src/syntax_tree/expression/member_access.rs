use crate::{Expression, Identifier, Span};

#[derive(Debug)]
pub struct MemberAccess {
    pub span: Span,
    pub source: Box<Expression>,
    pub member: Identifier,
}
