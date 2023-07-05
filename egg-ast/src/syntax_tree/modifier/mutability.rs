use crate::Span;

#[derive(Debug)]
pub struct MutabilityModifier {
    pub span: Span,
    pub value: MutabilityModifierValue,
}

#[derive(Debug)]
pub enum MutabilityModifierValue {
    Immutable,
    Mutable,
}
