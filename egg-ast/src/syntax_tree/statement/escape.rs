use crate::{Expression, Label, Span};

#[derive(Debug)]
pub struct EscapeStatement {
    pub span: Span,
    pub command: EscapeStatementCommand,
    pub label: Option<Label>,
    pub argument: Option<Expression>,
}

#[derive(Debug)]
pub enum EscapeStatementCommand {
    Return,
    Break,
    Continue,
}
