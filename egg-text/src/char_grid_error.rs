use derive_more::{Display, Error};

/// The grid doesn't have enough characters to match the requested index.
#[derive(Debug, Display, Clone, Copy, Error)]
#[display(fmt = "Character position does not exist")]
#[non_exhaustive]
pub struct CharPosOutOfBound;

/// The grid doesn't have enough lines to match the requested line index.
#[derive(Debug, Display, Clone, Copy, Error)]
#[display(fmt = "Line does not exist")]
#[non_exhaustive]
pub struct LnNumOutOfBound;

/// The line doesn't have enough characters to match the requested column index.
#[derive(Debug, Display, Clone, Copy, Error)]
#[display(fmt = "Column does not exist")]
#[non_exhaustive]
pub struct ColNumOutOfBound;

/// The grid does not contain the requested coordinate.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum LnColOutOfBound {
    /// The grid doesn't have enough lines to match the requested line index.
    #[display(fmt = "Line does not exist")]
    LineOutOfBound,
    /// The line doesn't have enough characters to match the requested column index.
    #[display(fmt = "Column does not exist")]
    ColumnOutOfBound,
}
