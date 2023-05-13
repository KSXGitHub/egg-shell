use super::common::parse_prefixed_number;
use crate::token::ParseMiddleToken;

/// The start of a binary token.
pub const BINARY_PREFIX: &str = "0b";

/// Token for integer in base-2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BinaryToken<Content> {
    pub body: Content,
}

const fn is_binary_body(char: &char) -> bool {
    matches!(char, '0' | '1' | '_')
}

impl<'a> ParseMiddleToken<&'a str> for BinaryToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (body, remaining) = parse_prefixed_number(input, BINARY_PREFIX, is_binary_body)?;
        let token = BinaryToken { body };
        Some((token, remaining))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn positive() {
        macro_rules! case {
            ($input:literal -> $token:literal, $remaining:literal) => {
                assert_eq!(
                    BinaryToken::parse($input).unwrap(),
                    (BinaryToken { body: $token }, $remaining),
                )
            };
        }

        case!("0b0" -> "0", "");
        case!("0b1" -> "1", "");
        case!("0b0123456789" -> "01", "23456789");
        case!("0b00100111i32" -> "00100111", "i32");
        case!("0b1001_1100_1101u64" -> "1001_1100_1101", "u64");
        case!("0b1110011_suffix" -> "1110011_", "suffix");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {
                assert_eq!(BinaryToken::parse($input), None)
            };
        }

        case!("");
        case!("0b");
        case!("_123");
        case!("u32");
    }
}
