use crate::{ArgumentList, Expression, Span};

#[derive(Debug)]
pub struct FunctionCallExpression {
    pub span: Span,
    pub function: Box<Expression>,
    pub arguments: ArgumentList,
}
