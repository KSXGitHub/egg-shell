use crate::{Expression, Label, Span, Statement};

#[derive(Debug)]
pub struct BlockExpression {
    pub span: Span,
    pub label: Option<Label>,
    pub initial_statements: Box<[Statement]>,
    pub return_value: Option<Box<Expression>>,
}
