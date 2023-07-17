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
    Expression,
}

#[derive(Debug)]
pub enum MetaFragSpecLiteral {
    Boolean,
    Float,
    Integer,
    Regex,
    String,
}