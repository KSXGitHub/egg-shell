use crate::{Attribute, Expression, OptionalIdentifier, Span, VisibilityModifier};

#[derive(Debug)]
pub struct ConstantDeclaration {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    pub visibility: Option<VisibilityModifier>,
    pub identifier: OptionalIdentifier,
    pub type_annotation: Option<Expression>,
    pub value: Option<Expression>,
}
