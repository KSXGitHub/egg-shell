use crate::{Identifier, Pattern};

#[derive(Debug)]
pub struct RenamePattern {
    pub key: Identifier,
    pub value: Pattern,
}
