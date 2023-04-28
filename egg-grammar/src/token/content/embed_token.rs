/// Token for a chunk of embedded lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbedToken<Content> {
    pub header: (QuoteType, Content),
    pub body: Vec<Content>,
}

/// Quote type of [`EmbedToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteType {
    /// Three single quotes (`'''`) were used to start the embedded block.
    Single,
    /// Three double quotes (`"""`) were used to start the embedded block.
    Double,
}
