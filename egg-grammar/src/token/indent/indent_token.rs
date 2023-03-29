use super::IndentChar;
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into, IntoIterator};
use std::fmt::{self, Debug, Formatter};

/// Token of indentation.
#[derive(Clone, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From, Into, IntoIterator)]
pub struct IndentToken(Vec<IndentChar>);

impl Debug for IndentToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "IndentToken ")?;

        for indent in self.iter() {
            write!(f, "<{}>", indent.abbr())?;
        }

        Ok(())
    }
}
