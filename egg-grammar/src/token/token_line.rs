use super::{ContentToken, IndentToken};

/// List of tokens from a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenLine<Content> {
    /// Token of the indentation at the start of the line.
    pub indent: IndentToken,
    /// List of [`ContentToken`] after indentation.
    pub content: Vec<ContentToken<Content>>,
}
