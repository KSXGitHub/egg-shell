use crate::{Annotation, Identifier, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub span: Span,
    pub annotations: Vec<Annotation>,
    pub header: ModuleHeader,
    pub body: Vec<ModuleItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleHeader {
    pub span: Span,
    pub identifier: Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleItem {}
