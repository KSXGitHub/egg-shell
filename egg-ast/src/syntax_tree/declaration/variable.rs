use crate::{Attribute, Expression, Pattern, Span};

#[derive(Debug)]
pub struct VariableDeclaration {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    pub binding_pattern: Pattern,
    pub type_annotation: Option<Expression>,
    pub value: Option<Expression>,
}
