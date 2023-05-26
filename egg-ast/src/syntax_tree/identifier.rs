use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub span: Span,
    pub lang: String,
    pub body: String,
}
