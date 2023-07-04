use crate::{SingleSegmentStringLiteral, Span};
use std::borrow::Cow;

#[derive(Debug)]
pub struct MultiSegmentStringLiteral {
    pub span: Span,
    pub separator: Cow<'static, str>,
    pub body: Box<[SingleSegmentStringLiteral]>,
}
