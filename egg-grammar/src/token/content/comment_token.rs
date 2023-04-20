/// Token for a line comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommentToken<Content> {
    pub content: Content,
}
