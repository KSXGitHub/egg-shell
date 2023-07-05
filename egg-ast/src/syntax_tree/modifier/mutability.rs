use crate::Span;

#[derive(Debug)]
pub struct Mutability {
    pub span: Span,
    pub value: MutabilityValue,
}

#[derive(Debug)]
pub enum MutabilityValue {
    Immutable,
    Mutable,
}
