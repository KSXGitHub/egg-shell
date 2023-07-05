mod boolean;
mod float;
mod integer;
mod regex;
mod string;

pub use boolean::*;
pub use float::*;
pub use integer::*;
pub use regex::*;
pub use string::*;

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
    String(StringLiteral),
    Regex(RegexLiteral),
}
