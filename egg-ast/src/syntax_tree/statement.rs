mod escape;

pub use escape::*;

use crate::{Declaration, Expression, Span};

#[derive(Debug)]
pub struct Statement {
    pub span: Span,
    pub body: StatementBody,
}

#[derive(Debug)]
pub enum StatementBody {
    Declaration(Declaration),
    Expression(Expression),
}
