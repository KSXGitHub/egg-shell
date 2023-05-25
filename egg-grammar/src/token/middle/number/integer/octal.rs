use crate::token::ParseMiddleToken;
use egg_common_utils::{is_number_body, parse_prefixed_number};

/// The start of an octal token.
pub const OCTAL_PREFIX: &str = "0o";

/// Token for integer in base-8.
///
/// **Structure:**
/// `0o <content>`
///
/// **Note:** To avoid weird syntax quirks and confusing error messages,
/// non-octal digits are allowed in this token, and it shall be the job
/// of the AST analyzer to detect them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OctalToken<Content>(pub Content);

impl<'a> ParseMiddleToken<&'a str> for OctalToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (body, rest) = parse_prefixed_number(input, OCTAL_PREFIX, is_number_body)?;
        let token = OctalToken(body);
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
                assert_eq!(OctalToken::parse($input), Some((OctalToken($token), $rest)));
            }};
        }

        case!("0o0" -> "0", "");
        case!("0o1" -> "1", "");
        case!("0o3" -> "3", "");
        case!("0o5" -> "5", "");
        case!("0o7" -> "7", "");
        case!("0o3657" -> "3657", "");
        case!("0o3657u32" -> "3657", "u32");
        case!("0o1234567i32" -> "1234567", "i32");
        case!("0o0123_4567u64" -> "0123_4567", "u64");
        case!("0o123_suffix" -> "123_", "suffix");
        case!("0o0123456789abcdef" -> "0123456789", "abcdef");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(OctalToken::parse($input), None);
            }};
        }

        case!("");
        case!("0");
        case!("0o");
        case!("_123");
        case!("u32");
    }
}
