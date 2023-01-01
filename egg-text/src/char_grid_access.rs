use std::{convert::Infallible, iter};

fn from_infallible<X>(infallible: Infallible) -> X {
    match infallible {}
}

type IntoOk<X> = fn(Result<X, Infallible>) -> X;

fn into_ok<X>(result: Result<X, Infallible>) -> X {
    result.unwrap_or_else(from_infallible)
}

/// Iterate over each character.
pub trait IterChar: TryIterChar<Error = Infallible> {
    /// Type of the resulting iterator.
    type CharIter: IntoIterator<Item = Self::Char>;
    /// Iterate over each character.
    fn iter_char(self) -> Self::CharIter;
}

impl<Grid> IterChar for Grid
where
    Grid: TryIterChar<Error = Infallible>,
{
    type CharIter = iter::Map<Self::CharResultIter, IntoOk<Self::Char>>;
    fn iter_char(self) -> Self::CharIter {
        self.try_iter_char().map(into_ok)
    }
}

/// Iterate over each character.
pub trait TryIterChar {
    /// Character type to be emitted on success.
    type Char;
    /// The associate error which is yielded on failure.
    type Error;
    /// Type of the resulting iterator.
    type CharResultIter: Iterator<Item = Result<Self::Char, Self::Error>>;
    /// Iterate over each character.
    fn try_iter_char(self) -> Self::CharResultIter;
}

/// Iterate over each line.
pub trait IterLine<'a>: TryIterLine<'a, Error = Infallible> {
    /// Type of the resulting iterator.
    type LineIter: Iterator<Item = Self::Line>;
    /// Iterate over each line.
    fn iter_line(&'a self) -> Self::LineIter;
}

impl<'a, Grid> IterLine<'a> for Grid
where
    Grid: TryIterLine<'a, Error = Infallible>,
{
    type LineIter = iter::Map<Self::LineResultIter, IntoOk<Self::Line>>;
    fn iter_line(&'a self) -> Self::LineIter {
        self.try_iter_line().map(into_ok)
    }
}

/// Iterate over each line.
pub trait TryIterLine<'a> {
    /// Type of item to be yielded on success.
    type Line;
    /// The associate error which is yielded on failure.
    type Error;
    /// Type of the resulting iterator.
    type LineResultIter: Iterator<Item = Result<Self::Line, Self::Error>>;
    /// Iterate over each line.
    fn try_iter_line(&'a self) -> Self::LineResultIter;
}

/// Get a character cell by coordinate.
pub trait CharAt<Coord> {
    /// Character type to return on success.
    type Char;
    /// The associate error which is returned on failure.
    type Error;
    /// Get a character cell by coordinate.
    fn char_at(self, coord: Coord) -> Result<Self::Char, Self::Error>;
}

/// Get a line of character cells by coordinate.
pub trait LineAt<LnNum> {
    /// Type of return value on success.
    type Line;
    /// The associate error which is returned on failure.
    type Error;
    /// Get a line of character cells by coordinate.
    fn line_at(self, ln_num: LnNum) -> Result<Self::Line, Self::Error>;
}

/// Get a slice from a start coordinate to the rest.
pub trait SliceFrom<Coord> {
    /// Type of return value on success.
    type Slice;
    /// The associate error which is returned on failure.
    type Error;
    /// Get a slice from a start coordinate to the rest.
    fn slice_from(self, start: Coord) -> Result<Self::Slice, Self::Error>;
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
