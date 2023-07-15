use crate::{Attribute, Span};

#[derive(Debug)]
pub struct DeclarativeMacro {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    // TODO
}
