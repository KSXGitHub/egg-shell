use crate::Span;

#[derive(Debug)]
pub struct SingleSegmentStringLiteral {
    pub span: Span,
    pub value: Box<str>,
}
