mod dict;
mod identifier;
mod rename;
mod tuple;

pub use dict::*;
pub use identifier::*;
pub use rename::*;
pub use tuple::*;

use crate::Span;

#[derive(Debug)]
pub struct Pattern {
    pub span: Span,
    pub body: PatternBody,
}

#[derive(Debug)]
pub enum PatternBody {
    Identifier(IdentifierPattern),
    Tuple(TuplePattern),
    Dict(DictPattern),
}
