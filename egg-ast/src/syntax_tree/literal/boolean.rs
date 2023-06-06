use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanLiteral {
    pub span: Span,
    pub value: bool,
}
