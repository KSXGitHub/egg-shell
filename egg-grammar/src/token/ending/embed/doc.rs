use crate::token::{ParseEmbedTokenTag, ParseMiddleToken, RawToken, WordToken};

/// Token for chunk of documentation lines.
pub type DocToken<Content> =
    super::EmbedToken<DocTokenTag<Content>, RawToken<Content>, RawToken<Content>>;

/// Tag of [`DocToken`].
///
/// **Structure:** `@@[name]`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocTokenTag<Content>(pub Option<WordToken<Content>>);

impl<'a> ParseEmbedTokenTag<&'a str> for DocTokenTag<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let input = input.strip_prefix("@@")?;
        let (name, rest) = match WordToken::parse(input) {
            Some((name, rest)) => (Some(name), rest),
            None => (None, input),
        };
        let token = DocTokenTag(name);
        Some((token, rest))
    }
}
