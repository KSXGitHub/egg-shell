use crate::token::ParseMiddleToken;
use split_first_char::split_first_char;

/// Token for a punctuation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PunctuationToken {
    Comma,
    Semicolon,
}

impl PunctuationToken {
    /// Lookup the corresponding character.
    pub const fn as_char(&self) -> char {
        match self {
            PunctuationToken::Comma => ',',
            PunctuationToken::Semicolon => ';',
        }
    }

    /// Convert a `char` into a [`PunctuationToken`].
    pub const fn from_char(char: char) -> Option<Self> {
        match char {
            ',' => Some(PunctuationToken::Comma),
            ';' => Some(PunctuationToken::Semicolon),
            _ => None,
        }
    }
}

impl<'a> ParseMiddleToken<&'a str> for PunctuationToken {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (char, rest) = split_first_char(input)?;
        let token = PunctuationToken::from_char(char)?;
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
            ($input:literal -> $token:ident, $rest:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(
                    PunctuationToken::parse($input),
                    Some((PunctuationToken::$token, $rest)),
                );
            }};
        }

        case!("," -> Comma, "");
        case!(";" -> Semicolon, "");
        case!(",," -> Comma, ",");
        case!(";;" -> Semicolon, ";");
        case!(",,," -> Comma, ",,");
        case!(";;;" -> Semicolon, ";;");
        case!(",a,b,c" -> Comma, "a,b,c");
        case!(";a;b;c" -> Semicolon, "a;b;c");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(PunctuationToken::parse($input), None);
            }};
        }

        case!("");
        case!(".");
        case!("a,b,c");
    }
}
