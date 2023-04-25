/// Token for a chunk of embedded lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbedToken<Content> {
    pub header: (EmbedTokenTag, Content),
    pub main_content: Vec<Content>,
}

/// Tag of an embed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbedTokenTag {
    Doc,
    Exec,
    String,
}
