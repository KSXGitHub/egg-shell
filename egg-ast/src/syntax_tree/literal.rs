mod boolean;
mod float;
mod integer;
mod multi_segment_string;
mod regex;
mod single_segment_string;

pub use boolean::*;
pub use float::*;
pub use integer::*;
pub use multi_segment_string::*;
pub use regex::*;
pub use single_segment_string::*;

use crate::Span;

#[derive(Debug)]
pub struct Literal {
    pub span: Span,
    pub body: LiteralBody,
}

#[derive(Debug)]
pub enum LiteralBody {
    Boolean(BooleanLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    SingleSegmentString(SingleSegmentStringLiteral),
    MultiSegmentString(SingleSegmentStringLiteral),
    Regex(RegexLiteral),
}
