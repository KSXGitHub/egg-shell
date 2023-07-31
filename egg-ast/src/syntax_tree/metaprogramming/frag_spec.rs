use crate::Span;

#[derive(Debug)]
pub struct MetaFragSpec {
    pub span: Span,
    pub value: MetaFragSpecValue,
}

#[derive(Debug)]
pub enum MetaFragSpecValue {
    Identifier,
    Literal(Option<MetaFragSpecLiteral>),
    Expression,  // NOTE: may change as the interface get fleshed out
    Declaration, // NOTE: may change as the interface get fleshed out
    Type,
    List(Box<MetaFragSpec>),
}

#[derive(Debug)]
pub enum MetaFragSpecLiteral {
    Boolean,
    Float,
    Integer,
    Regex,
    String,
}
