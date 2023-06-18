use crate::{Annotation, ModuleItem, Span};

#[derive(Debug)]
pub struct Program {
    pub span: Span,
    pub annotations: Box<[Annotation]>,
    pub body: Box<[ProgramItem]>,
}

#[derive(Debug)]
pub enum ProgramItem {
    Declaration(ModuleItem),
}
