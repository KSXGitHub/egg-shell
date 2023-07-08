use crate::{Expression, Span};

#[derive(Debug)]
pub struct Constraint {
    pub span: Span,
    pub body: Box<[Expression]>,
}
