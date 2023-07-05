use crate::Span;

#[derive(Debug)]
pub struct VisibilityModifier {
    pub span: Span,
    pub scope: VisibilityModifierScope,
}

#[derive(Debug)]
pub enum VisibilityModifierScope {
    Module,
    File,
    Package,
    Public,
}
