mod boolean;
mod integer;
mod real;

pub use boolean::*;
pub use integer::*;
pub use real::*;

use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub span: Span,
    pub body: LiteralBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralBody {
    Boolean(BooleanLiteral),
    Integer(IntegerLiteral),
    Real(RealLiteral),
    SingleSegmentString, // TODO: SingleSegmentStringLiteral
    MultiSegmentString,  // TODO: MultiSegmentStringLiteral
}
