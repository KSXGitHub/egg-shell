use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub span: Span,
}
