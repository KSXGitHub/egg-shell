use crate::{Identifier, Span};

#[derive(Debug)]
pub struct SimplePath {
    pub span: Span,
    pub body: Box<[Identifier]>,
}
