use crate::Span;
use std::borrow::Cow;

#[derive(Debug)]
pub struct StringLiteral {
    pub span: Span,
    pub value: Cow<'static, str>,
}
