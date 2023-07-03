use crate::{SingleSegmentStringLiteral, Span};

#[derive(Debug)]
pub struct MultiSegmentStringLiteral {
    pub span: Span,
    pub separator: Box<str>,
    pub body: Box<[SingleSegmentStringLiteral]>,
}
