use crate::{MutabilityModifier, OptionalIdentifier};

#[derive(Debug)]
pub struct IdentifierPattern {
    pub mutability: Option<MutabilityModifier>,
    pub identifier: OptionalIdentifier,
}
