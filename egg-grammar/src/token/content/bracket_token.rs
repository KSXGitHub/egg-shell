use crate::token::ParseSimpleToken;
use egg_common_utils::split_first_char;

/// Token for bracket characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BracketToken {
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

impl<'a> ParseSimpleToken<&'a str> for BracketToken {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        use BracketDirection::*;
        use BracketShape::*;
        let (first, rest) = split_first_char(input)?;
        let (direction, shape) = match first {
            '(' => (Open, Round),
            ')' => (Close, Round),
            '[' => (Open, Square),
            ']' => (Close, Square),
            '{' => (Open, Curly),
            '}' => (Close, Curly),
            _ => return None,
        };
        let token = BracketToken { direction, shape };
        Some((token, rest))
    }
}
