/// Token for chunk of documentation lines.
pub type DocToken<Content> = super::EmbedToken<DocTokenTag, Content, Content>;

/// Tag of [`DocToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocTokenTag;
