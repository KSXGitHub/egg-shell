use crate::{ArgumentList, Literal, OptionalIdentifier, Span};

#[derive(Debug)]
pub struct Expression {
    pub span: Span,
    pub body: ExpressionBody,
}

#[derive(Debug)]
pub enum ExpressionBody {
    Literal(Literal),
    Identifier(OptionalIdentifier),
    FunctionCall(FunctionalCallExpression),
}

#[derive(Debug)]
pub struct FunctionalCallExpression {
    pub span: Span,
    pub function: Box<Expression>,
    pub arguments: ArgumentList,
}
