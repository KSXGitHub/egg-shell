use crate::{Expression, Pattern, Span};

#[derive(Debug)]
pub struct TuplePattern {
    pub span: Span,
    pub head: Option<Expression>,
    pub body: Box<[Pattern]>,
}
