use crate::{Attribute, Expression, Pattern, RenamePattern, Span};

#[derive(Debug)]
pub struct ParameterList {
    pub span: Span,
    pub body: Box<[Parameter]>,
}

#[derive(Debug)]
pub struct Parameter {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    pub binding_pattern: ParameterBindingPattern,
    pub type_annotation: Option<Expression>,
    pub default_value: Option<Expression>,
}

#[derive(Debug)]
pub enum ParameterBindingPattern {
    Named(Pattern),
    Positional(RenamePattern),
}
