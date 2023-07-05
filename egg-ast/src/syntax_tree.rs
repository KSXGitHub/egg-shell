mod argument;
mod attribute;
mod expression;
mod identifier;
mod literal;
mod module;
mod mutability;
mod parameter;
mod path;
mod pattern;
mod program;
mod variable;
mod visibility;

pub use argument::*;
pub use attribute::*;
pub use expression::*;
pub use identifier::*;
pub use literal::*;
pub use module::*;
pub use mutability::*;
pub use parameter::*;
pub use path::*;
pub use pattern::*;
pub use program::*;
pub use variable::*;
pub use visibility::*;

use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};

/// Abstract syntax tree.
#[derive(Debug, AsMut, AsRef, Deref, DerefMut, From, Into)]
pub struct SyntaxTree(pub Program);
