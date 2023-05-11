/// Token of multi-line string.
pub type TextToken<Content> = super::EmbedToken<TextTokenTag, Content, Content>;

/// Tag and quote type of [`TextToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTokenTag {
    /// Three single quotes (`'''`) were used to start the embedded block.
    Single,
    /// Three double quotes (`"""`) were used to start the embedded block.
    Double,
}
