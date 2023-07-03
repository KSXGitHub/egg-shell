use crate::Span;
use egg_data::AstRegex;

#[derive(Debug)]
pub struct RegexLiteral {
    pub span: Span,
    pub value: AstRegex,
}
