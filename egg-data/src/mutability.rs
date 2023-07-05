/// Whether the subject can be modified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mutability {
    /// The subject cannot be modified.
    Immutable,
    /// The subject can be modified.
    Mutable,
}
