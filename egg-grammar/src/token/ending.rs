mod comment;
pub mod embed;

pub use comment::CommentToken;
pub use embed::{DocToken, TextToken};

/// Token at the end of a line.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EndingToken<Content> {
    /// Single line comment.
    ///
    /// A single line comment starts with the character `#`.
    Comment(CommentToken<Content>),

    /// Multi-line string or embedded code.
    String(TextToken<Content>),

    /// Documentation annotation.
    Doc(DocToken<Content>),
}
