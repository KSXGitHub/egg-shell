/// Token for bracket characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BracketRawToken {
    direction: BracketDirection,
    shape: BracketShape,
}

/// Open bracket or close bracket?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketDirection {
    /// When the character is one of `(`, `[`, `{`.
    Open,
    /// When the character is one of `)`, `]`, `}`.
    Close,
}

/// Round bracket, square bracket, or curly bracket?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketShape {
    /// Either `(` or `)`.
    Round,
    /// Either `[` or `]`.
    Square,
    /// Either `{` or `}`.
    Curly,
}
