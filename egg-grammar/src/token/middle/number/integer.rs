mod binary;
mod common;
mod decimal;
mod hexadecimal;
mod octal;

pub use binary::*;
pub use decimal::*;
pub use hexadecimal::*;
pub use octal::*;

use crate::token::ParseMiddleToken;

/// Token for integer number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerToken<Content> {
    Decimal(DecimalToken<Content>),
    Binary(BinaryToken<Content>),
    Octal(OctalToken<Content>),
    Hexadecimal(HexadecimalToken<Content>),
}

impl<Content> IntegerToken<Content> {
    /// Get prefix corresponds to the type of token.
    pub const fn prefix(&self) -> Option<&'static str> {
        Some(match self {
            IntegerToken::Decimal(_) => return None,
            IntegerToken::Binary(_) => BINARY_PREFIX,
            IntegerToken::Octal(_) => OCTAL_PREFIX,
            IntegerToken::Hexadecimal(_) => HEXADECIMAL_PREFIX,
        })
    }
}

impl<'a> ParseMiddleToken<&'a str> for IntegerToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        macro_rules! case {
            ($token_type:ident -> $token_variant:ident) => {
                if let Some((token, remaining)) = $token_type::parse(input) {
                    return Some((IntegerToken::$token_variant(token), remaining));
                }
            };
        }
        case!(DecimalToken -> Decimal);
        case!(HexadecimalToken -> Hexadecimal);
        case!(OctalToken -> Octal);
        case!(BinaryToken -> Binary);
        None
    }
}
