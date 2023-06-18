use crate::{Annotation, Identifier, ParameterList, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub span: Span,
    pub annotations: Box<[Annotation]>,
    pub header: ModuleHeader,
    pub body: Box<[ModuleItem]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleHeader {
    pub span: Span,
    pub identifier: Option<Identifier>,
    pub parameters: Option<ParameterList>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleItem {}
