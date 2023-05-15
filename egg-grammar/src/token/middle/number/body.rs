use super::{FractionalToken, IntegerToken};
use crate::token::ParseMiddleToken;
use derive_more::{From, TryInto};

#[derive(Debug, Clone, Copy, PartialEq, Eq, From, TryInto)]
pub enum NumberTokenBody<Content> {
    Integer(IntegerToken<Content>),
    Fractional(FractionalToken<Content>),
}

impl<'a> ParseMiddleToken<&'a str> for NumberTokenBody<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        macro_rules! case {
            ($token_type:ident) => {
                if let Some((token, rest)) = $token_type::parse(input) {
                    return Some((NumberTokenBody::from(token), rest));
                }
            };
        }
        case!(IntegerToken);
        case!(FractionalToken);
        None
    }
}
