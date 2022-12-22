use crate::{CharCell, CharCoord, Ordinal};

/// Iterate over each character.
pub trait TryIterChar<'a>: TryIterLoadChar<'a> {
    /// The associate error which is yielded on failure.
    type Error;
    /// Type of the resulting iterator.
    type CharResultIter: Iterator<Item = Result<CharCell, <Self as TryIterChar<'a>>::Error>> + 'a;
    /// Iterate over each character.
    fn try_iter_char(&'a self) -> Self::CharResultIter;
}

/// Iterate over and load each character.
pub trait TryIterLoadChar<'a> {
    /// The associate error which is yielded on failure.
    type Error;
    /// Type of the resulting iterator.
    type CharResultLoadIter: Iterator<Item = Result<CharCell, Self::Error>> + 'a;
    /// Iterate over and load each character.
    fn try_iter_load_char(&'a mut self) -> Self::CharResultLoadIter;
}

/// Iterate over each line.
pub trait TryIterLine<'a>: TryIterLoadLine<'a> {
    /// The associate error which is yielded on failure.
    type Error;
    /// Type of the resulting iterator.
    type LineResultIter: Iterator<Item = Result<Self::Line, <Self as TryIterLine<'a>>::Error>>;
    /// Iterate over each line.
    fn try_iter_line(&'a self) -> Self::LineResultIter;
}

/// Iterate over and load each line.
pub trait TryIterLoadLine<'a> {
    /// Type of item to be yielded on success.
    type Line;
    /// The associate error which is yielded on failure.
    type Error;
    /// Type of the resulting iterator.
    type LineResultLoadIter: Iterator<Item = Result<Self::Line, Self::Error>>;
    /// Iterate over and load each line.
    fn try_iter_load_line(&'a mut self) -> Self::LineResultLoadIter;
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
