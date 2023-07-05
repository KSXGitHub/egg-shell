use crate::Span;

#[derive(Debug)]
pub struct Visibility {
    pub span: Span,
    pub reader: VisibilityScope,
    pub writer: VisibilityScope,
}

#[derive(Debug)]
pub struct VisibilityScope {
    pub base: VisibilityScopeBase,
    pub delta: u64,
}

#[derive(Debug)]
pub enum VisibilityScopeBase {
    Module,
    File,
    Package,
    Public,
}
