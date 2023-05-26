use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
    pub span: Span,
}
