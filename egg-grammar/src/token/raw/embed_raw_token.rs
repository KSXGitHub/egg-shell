/// Raw token for a chunk of embedded lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbedRawToken<Content> {
    pub tag: Content,
    pub main_content: Vec<Content>,
}
