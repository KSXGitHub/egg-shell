use crate::{
    Attribute, Constraint, Declaration, OptionalIdentifier, ParameterList, Span, VisibilityModifier,
};

#[derive(Debug)]
pub struct ModuleDeclaration {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    pub header: ModuleDeclarationHeader,
    pub body: Box<[Declaration]>,
}

#[derive(Debug)]
pub struct ModuleDeclarationHeader {
    pub span: Span,
    pub visibility: Option<VisibilityModifier>,
    pub identifier: OptionalIdentifier,
    pub templates: Box<[ParameterList]>,
    pub parameters: Option<ParameterList>,
    pub constraint: Option<Constraint>,
}
