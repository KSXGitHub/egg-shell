use super::{FractionalToken, IntegerToken};
use crate::token::ParseMiddleToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberTokenBody<Content> {
    Integer(IntegerToken<Content>),
    Fractional(FractionalToken<Content>),
}

impl<'a> ParseMiddleToken<&'a str> for NumberTokenBody<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        macro_rules! case {
            ($token_type:ident -> $token_variant:ident) => {
                if let Some((token, rest)) = $token_type::parse(input) {
                    return Some((NumberTokenBody::$token_variant(token), rest));
                }
            };
        }
        case!(IntegerToken -> Integer);
        case!(FractionalToken -> Fractional);
        None
    }
}
