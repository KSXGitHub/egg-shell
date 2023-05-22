use crate::token::ParseMiddleToken;

/// Token of a string of spaces and tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhitespaceToken<Content>(pub Content);

pub const fn is_whitespace(char: &char) -> bool {
    matches!(char, ' ' | '\t')
}

impl<'a> ParseMiddleToken<&'a str> for WhitespaceToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let end_offset = input.chars().take_while(is_whitespace).count(); // both space and tab has len_utf8 = 1
        if end_offset == 0 {
            return None;
        }
        let content = &input[..end_offset];
        let rest = &input[end_offset..];
        let token = WhitespaceToken(content);
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
                    WhitespaceToken::parse($input),
                    Some((WhitespaceToken($token), $rest)),
                );
            }};
        }
        case!(" " -> " ", "");
        case!("\t" -> "\t", "");
        case!("  " -> "  ", "");
        case!("\t\t" -> "\t\t", "");
        case!(" \t \t" -> " \t \t", "");
        case!(" abc" -> " ", "abc");
        case!("\tabc" -> "\t", "abc");
        case!("  abc" -> "  ", "abc");
        case!("\t\tabc" -> "\t\t", "abc");
        case!(" \t \tabc" -> " \t \t", "abc");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(WhitespaceToken::parse($input), None);
            }};
        }
        case!("");
        case!("\n"); // parse by line so newline will have no chance to appear
        case!("abc");
    }
}
