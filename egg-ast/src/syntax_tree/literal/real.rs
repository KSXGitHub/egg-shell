use crate::Span;
use num_bigint::{BigInt, BigUint, Sign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealLiteral {
    pub span: Span,
    pub sign: Sign,
    pub integer: BigUint,
    pub fractional: BigUint,
    pub exponent: BigInt,
    pub precision: Option<RealLiteralPrecision>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealLiteralPrecision {
    Float32,
    Float64,
    Arbitrary,
}
