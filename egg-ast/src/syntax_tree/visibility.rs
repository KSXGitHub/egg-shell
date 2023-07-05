use crate::Span;

#[derive(Debug)]
pub struct Visibility {
    pub span: Span,
    pub reader: VisibilityScope,
    pub writer: VisibilityScope,
}

#[derive(Debug)]
pub enum VisibilityScope {
    Module,
    File,
    Package,
    Public,
}
