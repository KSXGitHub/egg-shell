use crate::Span;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Label {
    pub span: Span,
    pub name: Cow<'static, str>,
}
