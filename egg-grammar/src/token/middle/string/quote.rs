/// Quote type of [`StringToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quote {
    /// Single quotes (`'`) were used to wrap the string content.
    Single,
    /// Double quotes (`"`) were used to wrap the string content.
    Double,
}
