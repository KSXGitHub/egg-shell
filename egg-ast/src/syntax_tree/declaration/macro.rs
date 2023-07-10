use crate::{Attribute, Span};

#[derive(Debug)]
pub struct MacroDeclaration {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    // TODO
}
