use crate::{Expression, Identifier, Span};

#[derive(Debug)]
pub struct FieldAccess {
    pub span: Span,
    pub source: Box<Expression>,
    pub field: Identifier,
}
