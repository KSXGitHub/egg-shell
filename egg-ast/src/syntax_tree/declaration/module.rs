use crate::{
    Attribute, OptionalIdentifier, ParameterList, Span, VariableDeclaration, VisibilityModifier,
};

#[derive(Debug)]
pub struct ModuleDeclaration {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    pub header: ModuleDeclarationHeader,
    pub body: Box<[ModuleItem]>,
}

#[derive(Debug)]
pub struct ModuleDeclarationHeader {
    pub span: Span,
    pub visibility: Option<VisibilityModifier>,
    pub identifier: OptionalIdentifier,
    pub parameters: Option<ParameterList>,
}

#[derive(Debug)]
pub enum ModuleItem {
    Module(ModuleDeclaration),
    Variable(VariableDeclaration),
}
