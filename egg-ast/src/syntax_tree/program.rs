use crate::{Attribute, Declaration, Span};

#[derive(Debug)]
pub struct Program {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    pub body: Box<[ProgramItem]>,
}

#[derive(Debug)]
pub enum ProgramItem {
    Declaration(Declaration),
}
