use crate::Ordinal;
use derive_more::Display;

/// Coordinate of a character.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
#[display(fmt = "{line}:{column}")]
pub struct CharCoord {
    /// Line number of the character.
    pub line: Ordinal,
    /// Column number of the character.
    pub column: Ordinal,
}

impl CharCoord {
    /// Create a character coordinate.
    pub const fn new(line: Ordinal, column: Ordinal) -> Self {
        CharCoord { line, column }
    }

    /// Create a character coordinate from line and column offsets.
    pub const fn from_offsets(ln_offset: usize, col_offset: usize) -> Self {
        Self::new(
            Ordinal::from_offset(ln_offset),
            Ordinal::from_offset(col_offset),
        )
    }
}
