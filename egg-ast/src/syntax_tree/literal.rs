use crate::Span;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub span: Span,
    pub body: LiteralBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralBody {
    Boolean(BooleanLiteral),
    Integer(IntegerLiteral),
    Fractional,          // TODO: FractionalLiteral
    SingleSegmentString, // TODO: SingleSegmentStringLiteral
    MultiSegmentString,  // TODO: MultiSegmentStringLiteral
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanLiteral {
    pub span: Span,
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub span: Span,
    pub value: BigInt,
}
