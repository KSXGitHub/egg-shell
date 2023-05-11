use crate::token::ParseMiddleToken;

/// Token for a line comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommentToken<Content> {
    pub content: Content,
}

impl<Content> CommentToken<Content> {
    /// If `content` is a comment, return a token in a [`Some`].
    /// Otherwise, return a [`None`].
    pub fn try_from_str(content: Content) -> Option<Self>
    where
        Content: AsRef<str>,
    {
        if content.as_ref().starts_with('#') {
            Some(CommentToken { content })
        } else {
            None
        }
    }
}

impl<'a> ParseMiddleToken<&'a str> for CommentToken<&'a str> {
    fn parse(content: &'a str) -> Option<(Self, &'a str)> {
        let token = CommentToken::try_from_str(content)?;
        Some((token, ""))
    }
}