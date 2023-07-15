mod attribute;
mod declaration;
mod expression;
mod identifier;
mod literal;
mod metaprogramming;
mod miscellaneous;
mod modifier;
mod path;
mod pattern;
mod program;
mod statement;

pub use attribute::*;
pub use declaration::*;
pub use expression::*;
pub use identifier::*;
pub use literal::*;
pub use metaprogramming::*;
pub use miscellaneous::*;
pub use modifier::*;
pub use path::*;
pub use pattern::*;
pub use program::*;
pub use statement::*;

use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};

/// Abstract syntax tree.
#[derive(Debug, AsMut, AsRef, Deref, DerefMut, From, Into)]
pub struct SyntaxTree(pub Program);
