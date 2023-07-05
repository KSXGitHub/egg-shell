use crate::Span;
use egg_data::VisibilityScope;

#[derive(Debug)]
pub struct VisibilityModifier {
    pub span: Span,
    pub scope: VisibilityScope,
}
