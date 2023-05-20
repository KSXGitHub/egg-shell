/// Token for a line comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommentToken<Content>(pub Content);

impl<'a> CommentToken<&'a str> {
    /// Parse an input text into a line comment.
    ///
    /// **Note:** `line` is assumed to not contain any EOL characters.
    pub fn parse(input: &'a str) -> Option<Self> {
        input.strip_prefix('#').map(CommentToken)
    }
}
