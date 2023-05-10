/// Token for chunk of documentation lines.
pub type DocumentationToken<Content> = super::EmbedToken<DocumentationTokenTag, Content, Content>;

/// Tag of [`DocumentationToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocumentationTokenTag;
