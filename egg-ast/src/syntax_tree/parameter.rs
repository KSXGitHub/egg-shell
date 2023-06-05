use crate::{SinglePattern, SingleRenamePattern, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterList {
    pub span: Span,
    pub body: Vec<Parameter>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub span: Span,
    pub binding_form: ParameterBindingForm,
    pub data_type: Option<()>,     // TODO: Expression
    pub default_value: Option<()>, // TODO: Expression
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterBindingForm {
    Named(SingleRenamePattern),
    Positional(SinglePattern),
}
