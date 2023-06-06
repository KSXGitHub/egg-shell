use crate::Span;
use num_bigint::{BigInt, BigUint};

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
