mod comment_token;
pub mod embed_token;

pub use comment_token::CommentToken;
pub use embed_token::EmbedToken;

/// Token at the end of a line.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EndingToken<Content> {
    /// Single line comment.
    ///
    /// A single line comment starts with the character `#`.
    Comment(CommentToken<Content>),

    /// Multi-line string or embedded code.
    Embed(EmbedToken<Content>),
}
