use crate::{Annotation, ModuleItem, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub span: Span,
    pub annotations: Vec<Annotation>,
    pub body: Vec<ProgramItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramItem {
    Declaration(ModuleItem),
}
