use crate::token::ParseMiddleToken;

/// Token for integer in base-10.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecimalToken<Content>(pub Content);

const fn is_number_body(char: &char) -> bool {
    matches!(char, '0'..='9' | '_')
}

impl<'a> ParseMiddleToken<&'a str> for DecimalToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let mut iter = input.chars();

        let first_char = iter.next()?;
        if !first_char.is_ascii_digit() {
            return None;
        }

        let first_char_len = 1; // because it is an ascii character.
        let tail_size = iter.take_while(is_number_body).count(); // digit always has len_utf8 = 1
        let end_offset = first_char_len + tail_size;

        let content = &input[..end_offset];
        let remaining = &input[end_offset..];
        let token = DecimalToken(content);
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
                    DecimalToken::parse($input).unwrap(),
                    (DecimalToken($token), $remaining),
                )
            };
        }

        case!("0" -> "0", "");
        case!("123i32" -> "123", "i32");
        case!("123_456_789u32" -> "123_456_789", "u32");
        case!("123_suffix" -> "123_", "suffix");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {
                assert_eq!(DecimalToken::parse($input), None)
            };
        }

        case!("");
        case!("_123");
        case!("u32");
    }
}
