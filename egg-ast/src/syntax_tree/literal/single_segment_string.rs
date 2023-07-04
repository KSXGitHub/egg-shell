use crate::Span;
use std::borrow::Cow;

#[derive(Debug)]
pub struct SingleSegmentStringLiteral {
    pub span: Span,
    pub value: Cow<'static, str>,
}
