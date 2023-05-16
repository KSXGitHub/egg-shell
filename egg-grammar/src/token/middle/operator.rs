use crate::token::ParseMiddleToken;

/// Token a sequence of special characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorToken<Content>(pub Content);

const fn is_operator_char(char: &char) -> bool {
    matches!(char, '!' | '%' | '&' | '*'..='/' | ':'..='?' | '\\' | '^' | '|' | '~')
}

impl<'a> ParseMiddleToken<&'a str> for OperatorToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let end_offset = input.chars().take_while(is_operator_char).count(); // ascii char has len_utf8 = 1
        let content = &input[..end_offset];
        let rest = &input[end_offset..];
        let token = OperatorToken(content);
        Some((token, rest))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_operator_chars() {
        macro_rules! case {
            ($char:literal) => {{
                let char = $char;
                eprintln!("TEST: {char:?}");
                assert!(!is_operator_char(&char));
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
}
