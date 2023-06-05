mod annotation;
mod expression;
mod identifier;
mod module;
mod parameter;
mod pattern;
mod program;

pub use annotation::*;
pub use expression::*;
pub use identifier::*;
pub use module::*;
pub use parameter::*;
pub use pattern::*;
pub use program::*;

use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};

/// Abstract syntax tree.
#[derive(Debug, Clone, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From, Into)]
pub struct SyntaxTree(pub Program);
