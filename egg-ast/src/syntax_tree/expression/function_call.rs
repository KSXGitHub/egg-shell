use crate::{Expression, FunctionArgumentList, Span};

#[derive(Debug)]
pub struct FunctionCallExpression {
    pub span: Span,
    pub callee: Box<Expression>,
    pub arguments: FunctionArgumentList,
}
