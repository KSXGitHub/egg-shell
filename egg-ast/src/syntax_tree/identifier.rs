use crate::{Name, OptionalName, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub span: Span,
    pub body: Name<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalIdentifier {
    pub span: Span,
    pub body: OptionalName<String, String>,
}
