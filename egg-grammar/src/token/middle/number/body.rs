use super::{IntegerNumber, RealNumber};
use crate::token::ParseMiddleToken;
use derive_more::{From, TryInto};
use pipe_trait::Pipe;

/// Body of a [number token](super::NumberToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, From, TryInto)]
pub enum NumberTokenBody<Content> {
    Integer(IntegerNumber<Content>),
    Fractional(RealNumber<Content>),
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
        try_parse!(RealNumber);
        try_parse!(IntegerNumber);
        None
    }
}

macro_rules! impl_from_int {
    ($token_type:ident) => {
        impl<Content> From<super::$token_type<Content>> for NumberTokenBody<Content> {
            fn from(token: super::$token_type<Content>) -> Self {
                token.pipe(IntegerNumber::from).into()
            }
        }
    };
}

impl_from_int!(DecimalInteger);
impl_from_int!(BinaryInteger);
impl_from_int!(OctalInteger);
impl_from_int!(HexadecimalInteger);
