mod error;
mod quote;

pub use error::*;
pub use quote::*;

use crate::token::ParseMiddleToken;
use egg_common_utils::{char_matcher, split_hbt_ascii};

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

fn parse_word(input: &str) -> (&'_ str, &'_ str) {
    char_matcher!(is_word_head => 'a'..='z' | 'A'..='Z' | '0'..='9' | '_');
    char_matcher!(is_word_body => 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-');
    char_matcher!(is_word_tail => 'a'..='z' | 'A'..='Z' | '0'..='9' | '_');
    split_hbt_ascii(input, is_word_head, is_word_body, is_word_tail)
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

            end_offset += char.len_utf8();

            if char == '\\' && !escaping {
                escaping = true;
                continue;
            }

            escaping = false;
        }

        let body = &input[start_offset..end_offset];

        if !end_quote {
            let suffix = "";
            let error = Some(Error::EndQuoteNotFound);
            let token = StringToken {
                prefix,
                suffix,
                body,
                quote,
                error,
            };
            let rest = &input[end_offset..];
            return Some((token, rest));
        }

        let input = &input[(end_offset + quote_len)..];
        let (suffix, rest) = parse_word(input);
        let error = None;

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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn positive() {
        macro_rules! case {
            ($input:literal -> $token:expr, $rest:literal) => {{
                eprintln!("TEST: {}", $input);
                assert_eq!(StringToken::parse($input), Some(($token, $rest)));
            }};
        }

        case!("''" -> StringToken {
            prefix: "",
            suffix: "",
            body: "",
            quote: Quote::Single,
            error: None,
        }, "");

        case!(r#""""# -> StringToken {
            prefix: "",
            suffix: "",
            body: "",
            quote: Quote::Double,
            error: None,
        }, "");

        case!("'' abc" -> StringToken {
            prefix: "",
            suffix: "",
            body: "",
            quote: Quote::Single,
            error: None,
        }, " abc");

        case!(r#""" abc"# -> StringToken {
            prefix: "",
            suffix: "",
            body: "",
            quote: Quote::Double,
            error: None,
        }, " abc");

        case!("'abc def ghi'" -> StringToken {
            prefix: "",
            suffix: "",
            body: "abc def ghi",
            quote: Quote::Single,
            error: None,
        }, "");

        case!(r#""abc def ghi""# -> StringToken {
            prefix: "",
            suffix: "",
            body: "abc def ghi",
            quote: Quote::Double,
            error: None,
        }, "");

        case!(r"'abc def \' ghi'" -> StringToken {
            prefix: "",
            suffix: "",
            body: r"abc def \' ghi",
            quote: Quote::Single,
            error: None,
        }, "");

        case!(r#""abc def \" ghi""# -> StringToken {
            prefix: "",
            suffix: "",
            body: r#"abc def \" ghi"#,
            quote: Quote::Double,
            error: None,
        }, "");

        case!("1foo-bar'abc def'" -> StringToken {
            prefix: "1foo-bar",
            suffix: "",
            body: "abc def",
            quote: Quote::Single,
            error: None,
        }, "");

        case!(r"prefix'abc def \' ghi\n\t'suffix++' jkl mno'" -> StringToken {
            prefix: "prefix",
            suffix: "suffix",
            body: r"abc def \' ghi\n\t",
            quote: Quote::Single,
            error: None,
        }, "++' jkl mno'");

        case!(r#"prefix"abc def \" ghi\n\t"suffix++' jkl mno'"# -> StringToken {
            prefix: "prefix",
            suffix: "suffix",
            body: r#"abc def \" ghi\n\t"#,
            quote: Quote::Double,
            error: None,
        }, "++' jkl mno'");

        case!("123-abc'def'ghi-jkl-" -> StringToken {
            prefix: "123-abc",
            suffix: "ghi-jkl",
            body: "def",
            quote: Quote::Single,
            error: None,
        }, "-");

        case!("prefix'I ❤️ programming.'suffíx" -> StringToken {
            prefix: "prefix",
            suffix: "suff",
            body: "I ❤️ programming.",
            quote: Quote::Single,
            error: None,
        }, "íx");

        case!("GalaxyBrainGigachadWarStratagem'三十六計，走為上計'逃げるんだよ" -> StringToken {
            prefix: "GalaxyBrainGigachadWarStratagem",
            suffix: "",
            body: "三十六計，走為上計",
            quote: Quote::Single,
            error: None
        }, "逃げるんだよ");

        case!("'" -> StringToken {
            prefix: "",
            suffix: "",
            body: "",
            quote: Quote::Single,
            error: Some(Error::EndQuoteNotFound),
        }, "");

        case!("\"" -> StringToken {
            prefix: "",
            suffix: "",
            body: "",
            quote: Quote::Double,
            error: Some(Error::EndQuoteNotFound),
        }, "");

        case!("'abc" -> StringToken {
            prefix: "",
            suffix: "",
            body: "abc",
            quote: Quote::Single,
            error: Some(Error::EndQuoteNotFound),
        }, "");

        case!("\"abc" -> StringToken {
            prefix: "",
            suffix: "",
            body: "abc",
            quote: Quote::Double,
            error: Some(Error::EndQuoteNotFound),
        }, "");

        case!("prefix'abc def ghi" -> StringToken {
            prefix: "prefix",
            suffix: "",
            body: "abc def ghi",
            quote: Quote::Single,
            error: Some(Error::EndQuoteNotFound),
        }, "");

        case!("prefix\"abc def ghi" -> StringToken {
            prefix: "prefix",
            suffix: "",
            body: "abc def ghi",
            quote: Quote::Double,
            error: Some(Error::EndQuoteNotFound),
        }, "");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {}", $input);
                assert_eq!(StringToken::parse($input), None);
            }};
        }

        case!("");
        case!("abc");
        case!("prefix-'abc'");
        case!("prefíx'abc'"); // í is not i
    }
}
