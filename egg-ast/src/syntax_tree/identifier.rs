use crate::{Name, OptionalName, Span};

#[derive(Debug)]
pub struct Identifier<Body = Name<String, String>> {
    pub span: Span,
    pub body: Body,
}

pub type OptionalIdentifier = Identifier<OptionalName<String, String>>;
