use crate::token::ParseMiddleToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberTokenSuffix<Content>(pub Content);

const fn is_number_suffix(char: &char) -> bool {
    matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

impl<'a> ParseMiddleToken<&'a str> for NumberTokenSuffix<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let end_offset = input.chars().take_while(is_number_suffix).count(); // ascii char has len_utf8 = 1, so count is safe
        if end_offset == 0 {
            return None;
        }
        let suffix = &input[..end_offset];
        let rest = &input[end_offset..];
        Some((NumberTokenSuffix(suffix), rest))
    }
}
