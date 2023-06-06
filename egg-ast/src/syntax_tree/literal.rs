mod boolean;
mod fraction;
mod integer;

pub use boolean::*;
pub use fraction::*;
pub use integer::*;

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
    Fraction(FractionLiteral),
    SingleSegmentString, // TODO: SingleSegmentStringLiteral
    MultiSegmentString,  // TODO: MultiSegmentStringLiteral
}
