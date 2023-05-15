mod body;
mod common;
mod fractional;
mod integer;
mod suffix;

pub use body::*;
pub use fractional::*;
pub use integer::*;
pub use suffix::*;

use crate::token::ParseMiddleToken;

/// Token for numeric literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberToken<Content> {
    pub body: NumberTokenBody<Content>,
    pub suffix: Option<NumberTokenSuffix<Content>>,
}

impl<'a> ParseMiddleToken<&'a str> for NumberToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (body, input) = NumberTokenBody::parse(input)?;
        let (suffix, rest) = match NumberTokenSuffix::parse(input) {
            None => (None, input),
            Some((suffix, rest)) => (Some(suffix), rest),
        };
        let token = NumberToken { body, suffix };
        Some((token, rest))
    }
}
