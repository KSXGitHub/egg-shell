mod annotation;
mod argument;
mod expression;
mod identifier;
mod literal;
mod module;
mod parameter;
mod pattern;
mod program;
mod variable;

pub use annotation::*;
pub use argument::*;
pub use expression::*;
pub use identifier::*;
pub use literal::*;
pub use module::*;
pub use parameter::*;
pub use pattern::*;
pub use program::*;
pub use variable::*;

use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};

/// Abstract syntax tree.
#[derive(Debug, AsMut, AsRef, Deref, DerefMut, From, Into)]
pub struct SyntaxTree(pub Program);
