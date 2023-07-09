use crate::{Name, OptionalName, Span};
use std::borrow::Cow;

#[derive(Debug)]
pub struct Identifier<Body = Name<Cow<'static, str>, Cow<'static, str>>> {
    pub span: Span,
    pub body: Body,
}

pub type OptionalIdentifier = Identifier<OptionalName<Cow<'static, str>, Cow<'static, str>>>;
