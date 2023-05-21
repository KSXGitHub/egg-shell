use crate::token::ParseMiddleToken;
use egg_common_utils::{char_matcher, parse_hb_ascii};

/// Token a sequence of special characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorToken<Content>(pub Content);

char_matcher!(is_operator_head => '!' | '$'..='&' | '*'..='/' | ':'..='@' | '\\' | '^' | '|' | '~');
char_matcher!(is_operator_body => '!' | '#'..='&' | '*'..='/' | ':'..='@' | '\\' | '^' | '|' | '~');

impl<'a> ParseMiddleToken<&'a str> for OperatorToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        parse_hb_ascii(OperatorToken, input, is_operator_head, is_operator_body)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not_operator_head() {
        macro_rules! case {
            ($char:literal) => {{
                let char = $char;
                eprintln!("TEST: {char:?}");
                assert!(!is_operator_head(&char));
            }};

            ($first:literal $($rest:literal)*) => {{
                case!($first);
                case!($($rest)*);
            }}
        }

        case!('"' '\''); // used in string literals
        case!('(' ')' '[' ']' '{' '}'); // used as bracket tokens
        case!('#'); // used in comments
    }

    #[test]
    fn not_operator_body() {
        macro_rules! case {
            ($char:literal) => {{
                let char = $char;
                eprintln!("TEST: {char:?}");
                assert!(!is_operator_body(&char));
            }};

            ($first:literal $($rest:literal)*) => {{
                case!($first);
                case!($($rest)*);
            }}
        }

        case!('"' '\''); // used in string literals
        case!('(' ')' '[' ']' '{' '}'); // used as bracket tokens
    }

    #[test]
    fn positive() {
        macro_rules! case {
            ($input:literal -> $token:literal, $rest:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(
                    OperatorToken::parse($input),
                    Some((OperatorToken($token), $rest)),
                );
            }};
        }

        case!("+123" -> "+", "123");
        case!("-123" -> "-", "123");
        case!("*123" -> "*", "123");
        case!("/123" -> "/", "123");
        case!(">123" -> ">", "123");
        case!("<123" -> "<", "123");
        case!("=123" -> "=", "123");
        case!(">=123" -> ">=", "123");
        case!("<=123" -> "<=", "123");
        case!("==123" -> "==", "123");
        case!("!=123" -> "!=", "123");
        case!("+ +" -> "+", " +");
        case!("+=123" -> "+=", "123");
        case!("+ =123" -> "+", " =123");
        case!("+#abc" -> "+#", "abc");
        case!("+ #abc" -> "+", " #abc");
        case!("++'abc'" -> "++", "'abc'");
        case!(r#"++"abc"# -> "++", r#""abc"#);
        case!("$interpolation" -> "$", "interpolation");
        case!("@attribute" -> "@", "attribute");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(OperatorToken::parse($input), None);
            }};
        }

        case!("");
        case!("# comment");
    }
}
