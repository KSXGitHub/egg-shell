mod comment;
pub mod embed;

pub use comment::CommentToken;
pub use embed::{DocToken, TextToken};

use super::IndentToken;
use derive_more::{From, TryInto};
use pipe_trait::Pipe;

/// Token at the end of a line.
#[derive(Debug, Clone, PartialEq, Eq, From, TryInto)]
#[non_exhaustive]
pub enum EndingToken<Content> {
    /// Single line comment.
    ///
    /// A single line comment starts with the character `#`.
    Comment(CommentToken<Content>),

    /// Multi-line string or embedded code.
    Text(TextToken<Content>),

    /// Documentation annotation.
    Doc(DocToken<Content>),
}

impl<'input> EndingToken<&'input str> {
    /// Build an [`EndingToken`] from start to finish.
    pub(crate) fn build<'header_indent>(
        header_indent: &'header_indent IndentToken,
        header_text: &'input str,
        mut next_line: impl FnMut() -> Option<&'input str>,
    ) -> Option<Self> {
        macro_rules! try_build {
            ($token_type:ident) => {
                if let Some(token) = $token_type::build(header_indent, header_text, &mut next_line)
                {
                    return token.pipe(EndingToken::from).pipe(Some);
                }
            };
        }

        if let Some(comment) = CommentToken::parse(header_text) {
            return comment.pipe(EndingToken::from).pipe(Some);
        }
        try_build!(TextToken);
        try_build!(DocToken);
        None
    }
}
