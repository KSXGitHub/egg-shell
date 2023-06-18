use crate::{Expression, SinglePattern, SingleRenamePattern, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterList {
    pub span: Span,
    pub body: Box<[Parameter]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub span: Span,
    pub binding_pattern: ParameterBindingPattern,
    pub data_type: Option<Expression>,
    pub default_value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterBindingPattern {
    Named(SingleRenamePattern),
    Positional(SinglePattern),
}
