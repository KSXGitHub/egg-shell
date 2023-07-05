use crate::Span;
use egg_data::Mutability;

#[derive(Debug)]
pub struct MutabilityModifier {
    pub span: Span,
    pub value: Mutability,
}
