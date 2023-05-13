use super::common::{is_number_body, parse_prefixed_number};
use crate::token::ParseMiddleToken;

/// The start of an octal token.
pub const OCTAL_PREFIX: &str = "0o";

/// Token for integer in base-8.
///
/// **Note:** To avoid weird syntax quirks and confusing error messages,
/// non-octal digits are allowed in this token, and it shall be the job
/// of the semantic layer to detect them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OctalToken<Content> {
    pub body: Content,
}

impl<'a> ParseMiddleToken<&'a str> for OctalToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (body, remaining) = parse_prefixed_number(input, OCTAL_PREFIX, is_number_body)?;
        let token = OctalToken { body };
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
                    OctalToken::parse($input).unwrap(),
                    (OctalToken { body: $token }, $remaining),
                )
            };
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
        case!("0o0123456789" -> "0123456789", "");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {
                assert_eq!(OctalToken::parse($input), None)
            };
        }

        case!("");
        case!("0");
        case!("0o");
        case!("_123");
        case!("u32");
    }
}
