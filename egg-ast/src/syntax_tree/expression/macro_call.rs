use crate::{Expression, MetaArgumentList, Span};

#[derive(Debug)]
pub struct MacroCallExpression {
    pub span: Span,
    pub callee: Box<Expression>,
    pub arguments: MetaArgumentList,
}
