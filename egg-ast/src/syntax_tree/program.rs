use crate::{Annotation, ModuleItem, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub span: Span,
    pub annotations: Box<[Annotation]>,
    pub body: Box<[ProgramItem]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramItem {
    Declaration(ModuleItem),
}
