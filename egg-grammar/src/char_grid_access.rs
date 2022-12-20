use crate::{CharCell, CharCoord, Ordinal};

/// Get a character cell by coordinate.
pub trait CharAt: LoadCharAt {
    /// The associate error which is returned on failure.
    type Error;
    /// Get a character cell by coordinate.
    fn char_at(&self, coord: CharCoord) -> Result<CharCell, <Self as CharAt>::Error>;
}

/// Load a character cell by coordinate.
pub trait LoadCharAt {
    /// The associate error which is returned on failure.
    type Error;
    /// Load character cell by coordinate.
    fn load_char_at(&mut self, coord: CharCoord) -> Result<CharCell, Self::Error>;
}

/// Get a line of character cells by coordinate.
pub trait LineAt: LoadLineAt {
    /// The associate error which is returned on failure.
    type Error;
    /// Get a line of character cells by coordinate.
    fn line_at(&self, ln_num: Ordinal) -> Result<Self::Line, <Self as LineAt>::Error>;
}

/// Load a line of character cells by coordinate.
pub trait LoadLineAt {
    /// Type of return value on success.
    type Line;
    /// The associate error which is returned on failure.
    type Error;
    // Load a line of character cells by coordinate.
    fn load_line_at(&mut self, ln_num: Ordinal) -> Result<Self::Line, Self::Error>;
}

/// Get the number of character cells.
pub trait CharCount {
    /// Get the number of character cells.
    fn char_count(&self) -> usize;
}

/// Get the number of lines.
pub trait LineCount {
    /// Get the number of lines.
    fn line_count(&self) -> usize;
}
