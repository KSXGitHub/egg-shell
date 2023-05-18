use crate::token::ParseMiddleToken;
use egg_common_utils::parse_prefixed_number;

/// The start of a hexadecimal token.
pub const HEXADECIMAL_PREFIX: &str = "0x";

/// Token for integer in base-16.
///
/// **Structure:**
/// `0x <content>`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HexadecimalToken<Content>(pub Content);

const fn is_hexadecimal_body(char: &char) -> bool {
    matches!(char, '0'..='9' | 'A'..='F' | 'a'..='f' | '_')
}

impl<'a> ParseMiddleToken<&'a str> for HexadecimalToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (body, rest) = parse_prefixed_number(input, HEXADECIMAL_PREFIX, is_hexadecimal_body)?;
        let token = HexadecimalToken(body);
        Some((token, rest))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn positive() {
        macro_rules! case {
            ($input:literal -> $token:literal, $rest:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(
                    HexadecimalToken::parse($input),
                    Some((HexadecimalToken($token), $rest)),
                )
            }};
        }

        case!("0x0" -> "0", "");
        case!("0xA" -> "A", "");
        case!("0xa" -> "a", "");
        case!("0xfu32" -> "f", "u32");
        case!("0xDeadBeef" -> "DeadBeef", "");
        case!("0x123ABCi32" -> "123ABC", "i32");
        case!("0x0123_abcd_ABCDu64" -> "0123_abcd_ABCD", "u64");
        case!("0xab123_suffix" -> "ab123_", "suffix");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(HexadecimalToken::parse($input), None);
            }};
        }

        case!("");
        case!("0");
        case!("0x");
        case!("_123");
        case!("u32");
    }
}
