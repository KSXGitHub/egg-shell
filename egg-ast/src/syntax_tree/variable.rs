use crate::{Attribute, Expression, SinglePattern, Span};

#[derive(Debug)]
pub struct Variable {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    pub binding_pattern: SinglePattern,
    pub data_type: Option<Expression>,
    pub value: Option<Expression>,
}
