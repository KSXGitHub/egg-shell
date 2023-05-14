use super::DecimalToken;
use crate::token::{number::is_number_body, ParseMiddleToken};
use split_first_char::split_first_char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FractionalToken<Content> {
    /// The integer part of the token.
    pub integer: Content,
    /// The fraction part of the token, placed after a dot, not including the dot.
    pub fraction: Option<Content>,
    /// The exponent part of the token, placed after `e`, not including `e`.
    pub exponent: Option<Content>,
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
    let tail_size = iter.take_while(is_number_body).count();
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
        let token = FractionalToken {
            integer,
            fraction,
            exponent,
        };
        Some((token, rest))
    }
}
