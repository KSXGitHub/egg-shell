use crate::{Annotation, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub span: Span,
    pub annotations: Vec<Annotation>,
    pub body: Vec<ModuleItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleItem {}
