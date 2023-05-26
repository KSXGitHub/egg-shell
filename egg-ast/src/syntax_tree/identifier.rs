use crate::{Name, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub span: Span,
    pub body: Name<String, String>,
}
