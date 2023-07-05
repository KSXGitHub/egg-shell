use crate::{SimplePath, Span};

#[derive(Debug)]
pub struct Attribute {
    pub span: Span,
    pub path: SimplePath,
}
