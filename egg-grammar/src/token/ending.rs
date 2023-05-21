mod comment;
pub mod embed;

pub use comment::CommentToken;
pub use embed::{DocToken, TextToken};

use derive_more::{From, TryInto};

/// Token at the end of a line.
#[derive(Debug, Clone, PartialEq, Eq, From, TryInto)]
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
