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
    pub binding_class: ParameterBindingClass,
    pub binding_data_type: Option<()>, // TODO: Expression
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterBindingForm {
    Named(Identifier),
    Positional(Option<Identifier>), // TODO: Replace Identifier with destructure pattern
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterBindingClass {
    Type,
    Const,
    Var,
    Module,
    Macro,
}
