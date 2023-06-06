use crate::{ArgumentList, Literal, OptionalIdentifier, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub span: Span,
    pub body: ExpressionBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionBody {
    Literal(Literal),
    Identifier(OptionalIdentifier),
    FunctionCall(FunctionalCallExpression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionalCallExpression {
    pub span: Span,
    pub function: Box<Expression>,
    pub arguments: ArgumentList,
}
