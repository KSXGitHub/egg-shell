use super::{ContentToken, IndentToken};
use egg_ast::LnCol;

/// List of tokens from a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenLine<Content> {
    /// Token of the indentation at the start of the line.
    pub indent: (LnCol, IndentToken),
    /// List of [`ContentToken`] after indentation.
    pub content: Vec<(LnCol, ContentToken<Content>)>,
}
