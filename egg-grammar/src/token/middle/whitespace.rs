use crate::token::ParseMiddleToken;
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use split_first_char::split_first_char;
use std::fmt::{self, Display, Formatter};

/// Whitespace character. Either a space or a tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhitespaceToken {
    Space,
    Tab,
}

impl From<WhitespaceToken> for char {
    fn from(input: WhitespaceToken) -> Self {
        match input {
            WhitespaceToken::Space => ' ',
            WhitespaceToken::Tab => '\t',
        }
    }
}

/// Error when failing to [convert](TryFrom) a `char` to an [`WhitespaceToken`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
#[display(fmt = "Cannot convert {_0:?} to a whitespace")]
pub struct WhitespaceTokenParseError(#[error(not(source))] char);

impl WhitespaceTokenParseError {
    /// Get the original input.
    pub const fn input(self) -> char {
        self.0
    }
}

impl TryFrom<char> for WhitespaceToken {
    type Error = WhitespaceTokenParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' => Ok(WhitespaceToken::Space),
            '\t' => Ok(WhitespaceToken::Tab),
            _ => value.pipe(WhitespaceTokenParseError).pipe(Err),
        }
    }
}

impl Display for WhitespaceToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        char::from(*self).fmt(f)
    }
}

impl<'a> ParseMiddleToken<&'a str> for WhitespaceToken {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (char, rest) = split_first_char(input)?;
        let token = char.try_into().ok()?;
        Some((token, rest))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn convert_whitespace_to_char() {
        assert_eq!(char::from(WhitespaceToken::Space), ' ');
        assert_eq!(char::from(WhitespaceToken::Tab), '\t');
    }

    #[test]
    fn convert_char_to_whitespace() {
        assert_eq!(
            ' '.pipe(WhitespaceToken::try_from).unwrap(),
            WhitespaceToken::Space
        );
        assert_eq!(
            '\t'.pipe(WhitespaceToken::try_from).unwrap(),
            WhitespaceToken::Tab
        );
        assert_eq!(
            'a'.pipe(WhitespaceToken::try_from).unwrap_err(),
            WhitespaceTokenParseError('a'),
        );
    }

    #[test]
    fn convert_back_forth() {
        macro_rules! char_to_char {
            ($char:literal) => {
                assert_eq!(
                    $char
                        .pipe(WhitespaceToken::try_from)
                        .unwrap()
                        .pipe(char::from),
                    $char,
                );
            };
        }

        macro_rules! whitespace_to_whitespace {
            ($name:ident) => {
                assert_eq!(
                    WhitespaceToken::$name
                        .pipe(char::from)
                        .pipe(WhitespaceToken::try_from)
                        .unwrap(),
                    WhitespaceToken::$name,
                );
            };
        }

        char_to_char!(' ');
        char_to_char!('\t');
        whitespace_to_whitespace!(Space);
        whitespace_to_whitespace!(Tab);
    }

    #[test]
    fn display_fmt() {
        assert_eq!(WhitespaceToken::Space.pipe(char::from).to_string(), " ");
        assert_eq!(WhitespaceToken::Tab.pipe(char::from).to_string(), "\t");
    }

    #[test]
    fn positive() {
        macro_rules! case {
            ($input:literal -> $token:ident, $rest:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(
                    WhitespaceToken::parse($input),
                    Some((WhitespaceToken::$token, $rest)),
                );
            }};
        }
        case!(" " -> Space, "");
        case!("\t" -> Tab, "");
        case!("  " -> Space, " ");
        case!("\t\t" -> Tab, "\t");
        case!(" abc" -> Space, "abc");
        case!("\tabc" -> Tab, "abc");
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
