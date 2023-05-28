use crate::{Identifier, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterList {
    pub span: Span,
    pub body: Vec<Parameter>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub span: Span,
    pub binding_form: ParameterBindingForm,
    pub binding_class: (),             // TODO
    pub binding_data_type: Option<()>, // TODO
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterBindingForm {
    Named(Identifier),
    Positional(Option<Identifier>), // TODO: Replace Identifier with destructure pattern
}
