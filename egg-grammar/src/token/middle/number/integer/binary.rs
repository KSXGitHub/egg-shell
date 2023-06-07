use crate::token::ParseMiddleToken;
use egg_common_utils::{is_number_body, parse_prefixed_number};

/// The start of a binary token.
pub const BINARY_PREFIX: &str = "0b";

/// Token for integer in base-2.
///
/// **Structure:**
/// `0b <content>`
///
/// **Note:** To avoid weird syntax quirks and confusing error messages,
/// non-binary digits are allowed in this token, and it shall be the job
/// of the AST analyzer to detect them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BinaryInteger<Content>(pub Content);

impl<'a> ParseMiddleToken<&'a str> for BinaryInteger<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (body, rest) = parse_prefixed_number(input, BINARY_PREFIX, is_number_body)?;
        let token = BinaryInteger(body);
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
                    BinaryInteger::parse($input),
                    Some((BinaryInteger($token), $rest)),
                );
            }};
        }

        case!("0b0" -> "0", "");
        case!("0b1" -> "1", "");
        case!("0b00100111i32" -> "00100111", "i32");
        case!("0b1001_1100_1101u64" -> "1001_1100_1101", "u64");
        case!("0b1110011_suffix" -> "1110011_", "suffix");
        case!("0b0123456789abcdef" -> "0123456789", "abcdef");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(BinaryInteger::parse($input), None);
            }};
        }

        case!("");
        case!("0b");
        case!("_123");
        case!("u32");
    }
}
