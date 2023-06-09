use crate::Span;
use egg_data::AstFraction;
use hex_wrapper::{Hex32, Hex64};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatLiteral {
    pub span: Span,
    pub value: FloatLiteralValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FloatLiteralValue {
    Float32(Hex32),
    Float64(Hex64),
    Ambiguous(AmbiguousFloatLiteralValue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmbiguousFloatLiteralValue {
    Nan,
    NegativeInfinity,
    PositiveInfinity,
    Any(AstFraction),
}
