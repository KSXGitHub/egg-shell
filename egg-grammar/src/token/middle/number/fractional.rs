use super::DecimalToken;
use crate::token::{number::common::is_number_body, ParseMiddleToken};
use split_first_char::split_first_char;

/// Token for fractional number.
///
/// **Structure:**
/// * `<integer>`
/// * `<integer> . <fraction>`
/// * `<integer> e <exponent>`
/// * `<integer> . <fraction> e <exponent>`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FractionalToken<Content> {
    /// The integer part of the token.
    pub integer: Content,
    /// The fraction part of the token, placed after a dot, not including the dot.
    pub fraction: Option<Content>,
    /// The exponent part of the token, placed after `e`, not including `e`.
    pub exponent: Option<Content>,
}

impl<Content> FractionalToken<Content> {
    /// Quickly and dirtily create a [`FractionalToken`] without specifying its field names.
    pub const fn new(
        integer: Content,
        fraction: Option<Content>,
        exponent: Option<Content>,
    ) -> Self {
        FractionalToken {
            integer,
            fraction,
            exponent,
        }
    }
}

fn parse_fraction(input: &str) -> Option<(&'_ str, &'_ str)> {
    let Some(('.', input)) = split_first_char(input) else {
        return None;
    };
    let (DecimalToken(fraction), rest) = DecimalToken::parse(input)?;
    Some((fraction, rest))
}

fn parse_exponent(input: &str) -> Option<(&'_ str, &'_ str)> {
    let Some(('e', input)) = split_first_char(input) else {
        return None;
    };

    let mut iter = input.chars();
    let first_char = iter.next()?;
    if first_char != '+' && first_char != '-' && !is_number_body(&first_char) {
        return None;
    }

    let first_char_len = 1; // because it is an ascii character.
    debug_assert_eq!(first_char_len, first_char.len_utf8());
    let tail_size = iter.take_while(is_number_body).count(); // digit always has len_utf8 = 1
    let end_offset = first_char_len + tail_size;

    let exponent = &input[..end_offset];
    let rest = &input[end_offset..];
    Some((exponent, rest))
}

impl<'a> ParseMiddleToken<&'a str> for FractionalToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (DecimalToken(integer), input) = DecimalToken::parse(input)?;
        let (fraction, input) = match parse_fraction(input) {
            None => (None, input),
            Some((fraction, rest)) => (Some(fraction), rest),
        };
        let (exponent, rest) = match parse_exponent(input) {
            None => (None, input),
            Some((exponent, rest)) => (Some(exponent), rest),
        };
        if fraction.is_none() && exponent.is_none() {
            return None;
        }
        let token = FractionalToken {
            integer,
            fraction,
            exponent,
        };
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
            ($input:literal -> $integer:literal, $fraction:expr, $exponent:expr, $rest:literal) => {{
                eprintln!("TEST: {:?}", $input);
                let token = FractionalToken::new($integer, $fraction, $exponent);
                assert_eq!(FractionalToken::parse($input), Some((token, $rest)));
            }};
        }

        case!("123.456" -> "123", Some("456"), None, "");
        case!("123.456f64" -> "123", Some("456"), None, "f64");
        case!("123e456" -> "123", None, Some("456"), "");
        case!("123e456f64" -> "123", None, Some("456"), "f64");
        case!("123e+456f64" -> "123", None, Some("+456"), "f64");
        case!("123e-456f64" -> "123", None, Some("-456"), "f64");
        case!("123.456e789" -> "123", Some("456"), Some("789"), "");
        case!("123.456e789f64" -> "123", Some("456"), Some("789"), "f64");
        case!("123.456e+789f64" -> "123", Some("456"), Some("+789"), "f64");
        case!("123.456e-789f64" -> "123", Some("456"), Some("-789"), "f64");
        case!("54_083.850_96e+71_326f128" -> "54_083", Some("850_96"), Some("+71_326"), "f128");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(FractionalToken::parse($input), None);
            }};
        }

        case!("");
        case!("123");
        case!("123.f64");
        case!("123.e456");
        case!("_123.456");
        case!("f64");
        case!("0x123.456");
    }
}
