use crate::token::ParseMiddleToken;
use std::str::Chars;

/// Token for a numeric literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberToken<Content> {
    pub content: Content,
}

/// Check if a character belongs to the body of a number token.
const fn is_number_body(char: char) -> bool {
    matches!(char, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_')
}

/// Parse a the body of a number.
fn parse_number_body(offset: usize, iter: &mut Chars) -> Option<usize> {
    let first_char = iter.next()?;
    if !first_char.is_ascii_digit() {
        return None;
    }

    let mut end_offset = offset + first_char.len_utf8();
    while let Some(char) = iter.next() {
        if is_number_body(char) {
            end_offset += char.len_utf8();
            continue;
        }

        if char == '.' {
            let offset = end_offset + char.len_utf8();
            if let Some(end_offset) = parse_number_body(offset, iter) {
                return Some(end_offset);
            }
        }

        break;
    }

    Some(end_offset)
}

impl<'a> ParseMiddleToken<&'a str> for NumberToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let mut iter = input.chars();
        let end_offset = parse_number_body(0, &mut iter)?;
        let content = input.get(..end_offset)?;
        let token = NumberToken { content };
        let rest = input.get(end_offset..)?;
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
            ($input:literal -> $token:literal, $remaining:literal) => {
                assert_eq!(
                    NumberToken::parse($input).unwrap(),
                    (NumberToken { content: $token }, $remaining),
                )
            };
        }

        case!("0" -> "0", "");
        case!("123" -> "123", "");
        case!("123u32" -> "123u32", "");
        case!("123u32" -> "123u32", "");
        case!("123_456u32" -> "123_456u32", "");
        case!("123.45f64" -> "123.45f64", "");
        case!("123.f64" -> "123", ".f64"); // be design, this is '(123).f64', not '123.0f64'
        case!("123.45.remaining" -> "123.45", ".remaining");
        case!("123+456" -> "123", "+456");
        case!("123.456,789" -> "123.456", ",789");
        case!("0x123ABCi32 remaining" -> "0x123ABCi32", " remaining");
        case!("123_456_789_suffix remaining" -> "123_456_789_suffix", " remaining");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {
                assert_eq!(NumberToken::parse($input), None)
            };
        }

        case!("");
        case!("_123");
        case!("abc");
        case!("i32");
        case!("f64");
        case!("-123"); // negative numbers are handled in semantic level, not token level
    }
}
