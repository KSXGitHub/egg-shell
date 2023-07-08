use crate::{Expression, Span, Statement};

#[derive(Debug)]
pub struct BlockExpression {
    pub span: Span,
    pub label: (), // TODO: BlockLabel
    pub initial_statements: Box<[Statement]>,
    pub return_value: Option<Box<Expression>>,
}
