use crate::token::ParseMiddleToken;
use egg_common_utils::{is_number_body, parse_hb_ascii};

/// Token for integer in base-10.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecimalInteger<Content>(pub Content);

impl<'a> ParseMiddleToken<&'a str> for DecimalInteger<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        parse_hb_ascii(DecimalInteger, input, char::is_ascii_digit, is_number_body)
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
                    DecimalInteger::parse($input),
                    Some((DecimalInteger($token), $rest)),
                )
            }};
        }

        case!("0" -> "0", "");
        case!("123i32" -> "123", "i32");
        case!("123_456_789u32" -> "123_456_789", "u32");
        case!("123_suffix" -> "123_", "suffix");
        case!("0123456789abcdef" -> "0123456789", "abcdef");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(DecimalInteger::parse($input), None);
            }};
        }

        case!("");
        case!("_123");
        case!("u32");
    }
}
