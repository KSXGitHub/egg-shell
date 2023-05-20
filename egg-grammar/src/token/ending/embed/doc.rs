use crate::token::{ParseEmbedTokenTag, RawToken};

/// Token for chunk of documentation lines.
pub type DocToken<Content> = super::EmbedToken<DocTokenTag, RawToken<Content>, RawToken<Content>>;

/// Tag of [`DocToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocTokenTag;

impl<'a> ParseEmbedTokenTag<&'a str> for DocTokenTag {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let rest = input.strip_prefix("@@")?;
        Some((DocTokenTag, rest))
    }
}
