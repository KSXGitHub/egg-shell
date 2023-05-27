use crate::{Identifier, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterList {
    pub span: Span,
    pub body: Vec<Parameter>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub span: Span,
    pub invoke_form: (),                  // TODO
    pub binding_class: (),                // TODO
    pub binding_name: Option<Identifier>, // TODO: Replace Identifier with destructure pattern
    pub binding_data_type: Option<()>,    // TODO
}
