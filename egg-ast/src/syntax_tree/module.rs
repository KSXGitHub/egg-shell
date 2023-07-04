use crate::{Annotation, OptionalIdentifier, ParameterList, Span};

#[derive(Debug)]
pub struct Module {
    pub span: Span,
    pub annotations: Box<[Annotation]>,
    pub header: ModuleHeader,
    pub body: Box<[ModuleItem]>,
}

#[derive(Debug)]
pub struct ModuleHeader {
    pub span: Span,
    pub identifier: OptionalIdentifier,
    pub parameters: Option<ParameterList>,
}

#[derive(Debug)]
pub enum ModuleItem {
    Module(Module),
}
