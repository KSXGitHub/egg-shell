mod annotation;
mod identifier;
mod module;
mod program;

pub use annotation::*;
pub use identifier::*;
pub use module::*;
pub use program::*;

use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};

/// Abstract syntax tree.
#[derive(Debug, Clone, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From, Into)]
pub struct SyntaxTree(pub Program);
