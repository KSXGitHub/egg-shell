use crate::{Annotation, OptionalIdentifier, ParameterList, Span, Variable, Visibility};

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
    pub visibility: Option<Visibility>,
    pub identifier: OptionalIdentifier,
    pub parameters: Option<ParameterList>,
}

#[derive(Debug)]
pub enum ModuleItem {
    Module(Module),
    Variable(Variable),
}
