mod body;
mod fractional;
mod integer;
mod suffix;

pub use body::*;
pub use fractional::*;
pub use integer::*;
pub use suffix::*;

use crate::token::ParseMiddleToken;

/// Token for numeric literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberToken<Content> {
    pub body: NumberTokenBody<Content>,
    pub suffix: Option<NumberTokenSuffix<Content>>,
}

impl<'a> ParseMiddleToken<&'a str> for NumberToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (body, input) = NumberTokenBody::parse(input)?;
        let (suffix, rest) = match NumberTokenSuffix::parse(input) {
            None => (None, input),
            Some((suffix, rest)) => (Some(suffix), rest),
        };
        let token = NumberToken { body, suffix };
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
            ($input:literal -> $body:expr, $suffix:expr, $rest:literal) => {{
                eprintln!("TEST: {:?}", $input);
                let token = NumberToken {
                    body: $body.into(),
                    suffix: $suffix.map(NumberTokenSuffix),
                };
                assert_eq!(NumberToken::parse($input), Some((token, $rest)));
            }};
        }

        case!("0" -> DecimalToken("0"), None, "");
        case!("0b" -> DecimalToken("0"), Some("b"), ""); // actual binary requires digits
        case!("0o" -> DecimalToken("0"), Some("o"), ""); // actual octal requires digits
        case!("0x" -> DecimalToken("0"), Some("x"), ""); // actual hexadecimal requires digits
        case!("123" -> DecimalToken("123"), None, "");
        case!("123u32" -> DecimalToken("123"), Some("u32"), "");
        case!("123_456u32" -> DecimalToken("123_456"), Some("u32"), "");
        case!("123.45f64" -> FractionalToken::new("123", Some("45"), None), Some("f64"), "");
        case!("123." -> DecimalToken("123"), None, ".");
        case!("123.f64" -> DecimalToken("123"), None, ".f64"); // by design, this is '(123).f64', not '123.0f64'
        case!("123.45.rest" -> FractionalToken::new("123", Some("45"), None), None, ".rest");
        case!("123+456" -> DecimalToken("123"), None, "+456");
        case!("123.456,789" -> FractionalToken::new("123", Some("456"), None), None, ",789");
        case!("0x123ABCi32 rest" -> HexadecimalToken("123ABC"), Some("i32"), " rest");
        case!("123_456_789_suffix rest" -> DecimalToken("123_456_789_"), Some("suffix"), " rest");
        case!("123.456e-789f64" -> FractionalToken::new("123", Some("456"), Some("-789")), Some("f64"), "");
        case!("123.456-789f64" -> FractionalToken::new("123", Some("456"), None), None, "-789f64");
        case!("54_083.850_96e+71_326f128" -> FractionalToken::new("54_083", Some("850_96"), Some("+71_326")), Some("f128"), "");
        case!("54_083.850_96e+71_326f128+337" -> FractionalToken::new("54_083", Some("850_96"), Some("+71_326")), Some("f128"), "+337");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {
                assert_eq!(NumberToken::parse($input), None)
            };
        }

        case!("");
        case!("_123");
        case!("abc");
        case!("i32");
        case!("f64");
        case!("-123"); // negative numbers are handled in semantic level, not token level
    }
}
