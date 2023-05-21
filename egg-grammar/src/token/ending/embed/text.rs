use crate::token::{ParseEmbedTokenTag, RawToken};

/// Token of multi-line string.
pub type TextToken<Content> = super::EmbedToken<TextTokenTag, RawToken<Content>, RawToken<Content>>;

/// Tag and quote type of [`TextToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTokenTag {
    /// Three single quotes (`'''`) were used to start the embedded block.
    Single,
    /// Three double quotes (`"""`) were used to start the embedded block.
    Double,
}

impl<'a> ParseEmbedTokenTag<&'a str> for TextTokenTag {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        macro_rules! try_parse {
            ($syntax:literal -> $token_variant:ident) => {
                if let Some(rest) = input.strip_prefix($syntax) {
                    return Some((TextTokenTag::$token_variant, rest));
                }
            };
        }
        try_parse!("'''" -> Single);
        try_parse!("\"\"\"" -> Double);
        None
    }
}
