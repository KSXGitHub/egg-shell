use crate::{
    Attribute, Constraint, Expression, OptionalIdentifier, ParameterList, Span, VisibilityModifier,
};

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    pub header: FunctionDeclarationHeader,
    pub body: Option<Expression>,
}

#[derive(Debug)]
pub struct FunctionDeclarationHeader {
    pub span: Span,
    pub visibility: Option<VisibilityModifier>,
    pub identifier: Option<OptionalIdentifier>,
    pub parameters: ParameterList,
    pub return_type: Option<Expression>,
    pub constraint: Constraint,
}
