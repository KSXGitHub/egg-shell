use crate::Span;

#[derive(Debug)]
pub struct SyntaxTypeAnnotation {
    pub span: Span,
    pub value: SyntaxType,
}

#[derive(Debug)]
pub enum SyntaxType {
    Identifier,
    Literal,
    Expression,
}
