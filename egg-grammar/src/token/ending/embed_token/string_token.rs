/// Token of multi-line string.
pub type StringToken<Content> = super::EmbedToken<StringTokenTag, Content, Content>;

/// Tag and quote type of [`StringToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringTokenTag {
    /// Three single quotes (`'''`) were used to start the embedded block.
    Single,
    /// Three double quotes (`"""`) were used to start the embedded block.
    Double,
}
