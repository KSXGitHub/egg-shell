use super::{FractionalToken, IntegerToken};
use crate::token::ParseMiddleToken;
use derive_more::{From, TryInto};
use pipe_trait::Pipe;

/// Body of a [number token](super::NumberToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, From, TryInto)]
pub enum NumberTokenBody<Content> {
    Integer(IntegerToken<Content>),
    Fractional(FractionalToken<Content>),
}

impl<'a> ParseMiddleToken<&'a str> for NumberTokenBody<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        macro_rules! try_parse {
            ($token_type:ident) => {
                if let Some((token, rest)) = $token_type::parse(input) {
                    return Some((NumberTokenBody::from(token), rest));
                }
            };
        }
        try_parse!(FractionalToken);
        try_parse!(IntegerToken);
        None
    }
}

macro_rules! impl_from_int {
    ($token_type:ident) => {
        impl<Content> From<super::$token_type<Content>> for NumberTokenBody<Content> {
            fn from(token: super::$token_type<Content>) -> Self {
                token.pipe(IntegerToken::from).into()
            }
        }
    };
}

impl_from_int!(DecimalToken);
impl_from_int!(BinaryToken);
impl_from_int!(OctalToken);
impl_from_int!(HexadecimalToken);
