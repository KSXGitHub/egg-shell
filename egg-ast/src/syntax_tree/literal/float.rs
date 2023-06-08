use crate::Span;
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
}
