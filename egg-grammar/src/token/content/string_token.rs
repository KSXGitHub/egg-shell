/// String-like token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringToken<Content> {
    pub prefix: Option<Content>,
    pub suffix: Option<Content>,
    pub body: Content,
    pub quote: QuoteType,
}

/// Quote type of [`StringToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteType {
    /// Single quotes (`'`) were used to wrap the string content.
    Single,
    /// Double quotes (`"`) were used to wrap the string content.
    Double,
}
