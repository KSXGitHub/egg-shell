use crate::{CharCell, CharCoord, Ordinal};

/// Iterate over each character.
pub trait IterChar<'a>: IterLoadChar<'a> {
    /// The associate error which is yielded on failure.
    type Error;
    /// Type of the resulting iterator.
    type CharIter: IntoIterator<Item = Result<CharCell, <Self as IterChar<'a>>::Error>> + 'a;
    /// Iterate over each character.
    fn iter_char(&'a self) -> Self::CharIter;
}

/// Iterate over and load each character.
pub trait IterLoadChar<'a> {
    /// The associate error which is yielded on failure.
    type Error;
    /// Type of the resulting iterator.
    type CharLoadIter: IntoIterator<Item = Result<CharCell, Self::Error>> + 'a;
    /// Iterate over and load each character.
    fn iter_load_char(&'a mut self) -> Self::CharLoadIter;
}

/// Iterate over each line.
pub trait IterLine<'a>: IterLoadLine<'a> {
    /// The associate error which is yielded on failure.
    type Error;
    /// Type of the resulting iterator.
    type LineIter: IntoIterator<Item = Result<Self::Line, <Self as IterLine<'a>>::Error>>;
    /// Iterate over each line.
    fn iter_line(&'a self) -> Self::LineIter;
}

/// Iterate over and load each line.
pub trait IterLoadLine<'a> {
    /// Type of item to be yielded on success.
    type Line;
    /// The associate error which is yielded on failure.
    type Error;
    /// Type of the resulting iterator.
    type LineLoadIter: IntoIterator<Item = Result<Self::Line, Self::Error>>;
    /// Iterate over and load each line.
    fn iter_load_line(&'a mut self) -> Self::LineLoadIter;
}

/// Get a character cell by coordinate.
pub trait CharAt<'a>: LoadCharAt<'a> {
    /// The associate error which is returned on failure.
    type Error;
    /// Get a character cell by coordinate.
    fn char_at(&'a self, coord: CharCoord) -> Result<CharCell, <Self as CharAt>::Error>;
}

/// Load a character cell by coordinate.
pub trait LoadCharAt<'a> {
    /// The associate error which is returned on failure.
    type Error;
    /// Load character cell by coordinate.
    fn load_char_at(&'a mut self, coord: CharCoord) -> Result<CharCell, Self::Error>;
}

/// Get a line of character cells by coordinate.
pub trait LineAt<'a>: LoadLineAt<'a> {
    /// The associate error which is returned on failure.
    type Error;
    /// Get a line of character cells by coordinate.
    fn line_at(&'a self, ln_num: Ordinal) -> Result<Self::Line, <Self as LineAt>::Error>;
}

/// Load a line of character cells by coordinate.
pub trait LoadLineAt<'a> {
    /// Type of return value on success.
    type Line;
    /// The associate error which is returned on failure.
    type Error;
    // Load a line of character cells by coordinate.
    fn load_line_at(&'a mut self, ln_num: Ordinal) -> Result<Self::Line, Self::Error>;
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
