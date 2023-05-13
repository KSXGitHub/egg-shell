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
                if let Some((token, rest)) = $token_type::parse(input) {
                    return Some((IntegerToken::$token_variant(token), rest));
                }
            };
        }
        case!(HexadecimalToken -> Hexadecimal);
        case!(OctalToken -> Octal);
        case!(BinaryToken -> Binary);
        case!(DecimalToken -> Decimal);
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn positive() {
        use IntegerToken::*;

        macro_rules! case {
            ($input:literal -> $prefix:expr, $token:expr, $rest:literal) => {{
                let (token, rest) = IntegerToken::parse($input).unwrap();
                assert_eq!((token.prefix(), token, rest), ($prefix, $token, $rest));
            }};
        }

        case!("123i32" -> None, Decimal(DecimalToken("123")), "i32");
        case!("123_456_i32" -> None, Decimal(DecimalToken("123_456_")), "i32");
        case!("101101u32" -> None, Decimal(DecimalToken("101101")), "u32");
        case!("123ABCu64" -> None, Decimal(DecimalToken("123")), "ABCu64");
        case!("0b101101u32" -> Some("0b"), Binary(BinaryToken("101101")), "u32");
        case!("0b10_1101_u32" -> Some("0b"), Binary(BinaryToken("10_1101_")), "u32");
        case!("0o123i8" -> Some("0o"), Octal(OctalToken("123")), "i8");
        case!("0x123ABCu64" -> Some("0x"), Hexadecimal(HexadecimalToken("123ABC")), "u64");
        case!("0x12_3A_BC_u64" -> Some("0x"), Hexadecimal(HexadecimalToken("12_3A_BC_")), "u64");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {
                assert_eq!(IntegerToken::parse($input), None)
            };
        }

        case!("");
        case!("_123");
        case!("u32");
    }
}
