use crate::token::ParseMiddleToken;

/// Token a sequence of special characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorToken<Content>(pub Content);

const fn is_operator_head(char: &char) -> bool {
    matches!(char, '!' | '%' | '&' | '*'..='/' | ':'..='?' | '\\' | '^' | '|' | '~')
}

const fn is_operator_body(char: &char) -> bool {
    is_operator_head(char) || matches!(char, '$' | '#' | '@')
}

impl<'a> ParseMiddleToken<&'a str> for OperatorToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let mut iter = input.chars();

        let first_char = iter.next()?;
        if !is_operator_head(&first_char) {
            return None;
        }

        let first_char_len = 1; // because it is an ascii character.
        debug_assert_eq!(first_char_len, first_char.len_utf8());
        let tail_size = iter.take_while(is_operator_body).count(); // ascii char has len_utf8 = 1
        let end_offset = first_char_len + tail_size;

        let content = &input[..end_offset];
        let rest = &input[end_offset..];
        let token = OperatorToken(content);
        Some((token, rest))
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
        case!('$'); // used as variable interpolation in strings and macros
        case!('#'); // used in comments
        case!('@'); // used in annotations, meta attributes, and macros
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
        case!("$interpolation");
        case!("# comment");
        case!("@attribute");
    }
}
