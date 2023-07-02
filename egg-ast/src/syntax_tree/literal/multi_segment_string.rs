use crate::{SingleSegmentStringLiteral, Span};

#[derive(Debug)]
pub struct MultiSegmentStringLiteral {
    pub span: Span,
    pub body: Box<[SingleSegmentStringLiteral]>,
}
