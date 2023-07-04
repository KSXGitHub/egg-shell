use crate::{Annotation, Expression, SinglePattern, Span};

#[derive(Debug)]
pub struct Variable {
    pub span: Span,
    pub annotations: Box<[Annotation]>,
    pub binding_pattern: SinglePattern,
    pub data_type: Option<Expression>,
    pub value: Option<Expression>,
}
