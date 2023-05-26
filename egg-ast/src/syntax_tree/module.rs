use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub span: Span,
    pub body: Vec<ModuleItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleItem {}
