/// Raw, string-like token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringRawToken<Content> {
    pub prefix: Option<Content>,
    pub suffix: Option<Content>,
    pub main_content: Content,
    pub quote_type: QuoteType,
}

/// Quote type of [`StringRawToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteType {
    /// Single quotes (`'`) were used to wrap the string content.
    Single,
    /// Double quotes (`"`) were used to wrap the string content.
    Double,
}
