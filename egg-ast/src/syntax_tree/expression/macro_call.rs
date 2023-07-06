use crate::{Expression, Span};

#[derive(Debug)]
pub struct MacroCallExpression {
    pub span: Span,
    pub callee: Box<Expression>,
    pub arguments: (), // TODO: MetaArgumentList
}
