use crate::token::{ParseEmbedTokenTag, ParseMiddleToken, RawToken, WordToken};

/// Token for chunk of documentation lines.
pub type DocToken<Content> =
    super::EmbedToken<DocTokenTag<Content>, RawToken<Content>, RawToken<Content>>;

/// Tag of [`DocToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocTokenTag<Content>(pub WordToken<Content>);

impl<'a> ParseEmbedTokenTag<&'a str> for DocTokenTag<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let input = input.strip_prefix("@@")?;
        let (name, rest) = WordToken::parse(input)?;
        let token = DocTokenTag(name);
        Some((token, rest))
    }
}
