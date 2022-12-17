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

    /// Create a character coordinate from line and column predecessor counts.
    pub const fn from_pred_counts(ln_pred: usize, col_pred: usize) -> Self {
        Self::new(
            Ordinal::from_pred_count(ln_pred),
            Ordinal::from_pred_count(col_pred),
        )
    }
}
