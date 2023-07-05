mod attribute;
mod declaration;
mod expression;
mod identifier;
mod literal;
mod miscellaneous;
mod modifier;
mod path;
mod pattern;
mod program;

pub use attribute::*;
pub use declaration::*;
pub use expression::*;
pub use identifier::*;
pub use literal::*;
pub use miscellaneous::*;
pub use modifier::*;
pub use path::*;
pub use pattern::*;
pub use program::*;

use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};

/// Abstract syntax tree.
#[derive(Debug, AsMut, AsRef, Deref, DerefMut, From, Into)]
pub struct SyntaxTree(pub Program);
