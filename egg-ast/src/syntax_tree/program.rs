use crate::{ModuleItem, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub span: Span,
    pub body: Vec<ProgramItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramItem {
    Declaration(ModuleItem),
}
