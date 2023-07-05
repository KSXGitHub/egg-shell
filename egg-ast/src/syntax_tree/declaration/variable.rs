use crate::{Attribute, Expression, SinglePattern, Span};

#[derive(Debug)]
pub struct VariableDeclaration {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    pub binding_pattern: SinglePattern,
    pub type_annotation: Option<Expression>,
    pub value: Option<Expression>,
}
