mod error;
mod quote;

pub use error::*;
pub use quote::*;

use crate::token::ParseMiddleToken;

/// String-like token.
///
/// **Structure:**
/// `<prefix> <quote> <body> <quote> <suffix>`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringToken<Content> {
    pub prefix: Content,
    pub suffix: Content,
    pub body: Content,
    pub quote: Quote,
    pub error: Option<Error>,
}

const fn is_word_head(char: &char) -> bool {
    matches!(char, 'a'..='z' | 'A'..='Z' | '_')
}

const fn is_word_body(char: &char) -> bool {
    is_word_head(char) || matches!(char, '0'..='9' | '-')
}

fn parse_word(input: &str) -> (&'_ str, &'_ str) {
    let mut iter = input.chars();

    let Some(first_char) = iter.next() else {
        return ("", input);
    };
    if !is_word_head(&first_char) {
        return ("", input);
    }

    let first_char_len = 1; // because it is an ascii character.
    debug_assert_eq!(first_char_len, first_char.len_utf8());
    let tail_size = iter.take_while(is_word_body).count(); // ascii char has len_utf8 = 1
    let end_offset = first_char_len + tail_size;

    let word = &input[..end_offset];
    let rest = &input[end_offset..];
    (word, rest)
}

impl<'a> ParseMiddleToken<&'a str> for StringToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (prefix, input) = parse_word(input);

        let mut iter = input.chars();

        let quote = iter.next().and_then(Quote::from_char)?;
        let quote_len = 1; // len_utf8 of both single quote and double quote is 1

        let start_offset = quote_len;
        let mut escaping = false;
        let mut end_offset = start_offset;
        let mut end_quote = false;

        for char in iter {
            if char == quote.to_char() && !escaping {
                end_quote = true;
                break;
            }

            if char == '\\' && !escaping {
                escaping = true;
                continue;
            }

            end_offset += char.len_utf8();
            escaping = false;
        }

        let body = &input[start_offset..end_offset];
        let error = if end_quote {
            None
        } else {
            Some(Error::EndQuoteNotFound)
        };

        let input = &input[end_offset..];
        let (suffix, rest) = parse_word(input);

        let token = StringToken {
            prefix,
            suffix,
            body,
            quote,
            error,
        };
        Some((token, rest))
    }
}
