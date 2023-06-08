use crate::Span;
use hex_wrapper::{Hex32, Hex64};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealLiteral {
    pub span: Span,
    pub value: RealLiteralValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RealLiteralValue {
    Float32(Hex32),
    Float64(Hex64),
}
