use crate::{Identifier, OptionalIdentifier, SinglePattern, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterList {
    pub span: Span,
    pub body: Vec<Parameter>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub span: Span,
    pub binding_form: ParameterBindingForm,
    pub binding_data_type: Option<()>, // TODO: Expression
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterBindingForm {
    Named(ParameterNamedBindingForm),
    Positional(OptionalIdentifier),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterNamedBindingForm {
    pub name: Identifier,
    pub rename: Option<SinglePattern>,
}
