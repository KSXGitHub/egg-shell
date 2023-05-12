use crate::token::ParseMiddleToken;

/// Token for a numeric literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberToken<Content> {
    pub content: Content,
}

/// Check if a character belongs to the body of a number token.
const fn is_number_body(char: char) -> bool {
    matches!(char, '0'..='9' | 'a'..='z' | 'A'..='Z' | '.' | '_')
}

impl<'a> ParseMiddleToken<&'a str> for NumberToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let mut iter = input.chars();
        let first_char = iter.next()?;
        if !first_char.is_ascii_digit() {
            return None;
        }
        let mut end_offset = first_char.len_utf8();
        for char in iter {
            if is_number_body(char) {
                end_offset += char.len_utf8();
            } else {
                break;
            }
        }
        let content = input.get(..end_offset)?;
        let token = NumberToken { content };
        let rest = input.get(end_offset..)?;
        Some((token, rest))
    }
}
