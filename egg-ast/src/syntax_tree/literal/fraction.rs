use crate::Span;
use num_bigint::{BigInt, BigUint, Sign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FractionLiteral {
    pub span: Span,
    pub sign: Sign,
    pub integer: BigUint,
    pub fractional: BigUint,
    pub exponent: BigInt,
    pub precision: Option<FractionLiteralPrecision>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FractionLiteralPrecision {
    Float32,
    Float64,
    Arbitrary,
}
