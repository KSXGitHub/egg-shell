/// Scope of visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisibilityScope {
    /// Visible to all code in the same module.
    Module,
    /// Visible to all code in the same file.
    File,
    /// Visible to all code in the same package.
    Package,
    /// Visible to all code.
    Public,
}
