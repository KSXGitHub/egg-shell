use crate::Span;
use num_bigint::{BigInt, BigUint, Sign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub span: Span,
    pub body: LiteralBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralBody {
    Boolean(BooleanLiteral),
    Integer(IntegerLiteral),
    Fraction(FractionLiteral),
    SingleSegmentString, // TODO: SingleSegmentStringLiteral
    MultiSegmentString,  // TODO: MultiSegmentStringLiteral
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanLiteral {
    pub span: Span,
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral<Value = IntegerLiteralValue> {
    pub span: Span,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerLiteralValue {
    Limited(LimitedIntegerLiteralValue),
    Arbitrary(ArbitraryIntegerLiteralValue),
}

pub type LimitedIntegerLiteral = IntegerLiteral<LimitedIntegerLiteralValue>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitedIntegerLiteralValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
}

pub type ArbitraryIntegerLiteral = IntegerLiteral<ArbitraryIntegerLiteralValue>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArbitraryIntegerLiteralValue {
    Signed(BigInt),
    Unsigned(BigUint),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FractionLiteral {
    pub span: Span,
    pub sign: Sign,
    pub integer: BigUint,
    pub fractional: BigUint,
    pub exponent: BigInt,
    pub precision: Option<()>, // TODO: FractionLiteralPrecision
}
