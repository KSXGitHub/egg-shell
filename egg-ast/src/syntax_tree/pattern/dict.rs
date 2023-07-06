use crate::{Expression, RenamePattern, Span};

#[derive(Debug)]
pub struct DictPattern {
    pub span: Span,
    pub head: Option<Expression>,
    pub body: Box<[RenamePattern]>,
}
