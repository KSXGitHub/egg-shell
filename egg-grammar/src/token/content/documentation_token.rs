/// Token for chunks of documentation lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentationToken<Content> {
    pub header: Content,
    pub body: Vec<Content>,
}
