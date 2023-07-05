use crate::Span;

#[derive(Debug)]
pub struct Visibility {
    pub span: Span,
    pub scope: VisibilityScope,
}

#[derive(Debug)]
pub enum VisibilityScope {
    Module,
    File,
    Package,
    Public,
}
