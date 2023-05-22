use crate::token::{ParseEmbedTokenTag, RawToken};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Token of multi-line string.
pub type TextToken<Content> = super::EmbedToken<TextTokenTag, RawToken<Content>, RawToken<Content>>;

/// Tag and quote type of [`TextToken`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // essential std traits
#[derive(AsRefStr, Display, EnumString, IntoStaticStr)] // essential strum traits
pub enum TextTokenTag {
    /// Three single quotes (`'''`) were used to start the embedded block.
    #[strum(serialize = "'''")]
    Single,
    /// Three double quotes (`"""`) were used to start the embedded block.
    #[strum(serialize = "\"\"\"")]
    Double,
}

impl<'a> ParseEmbedTokenTag<&'a str> for TextTokenTag {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        macro_rules! try_parse {
            ($token_variant:ident) => {
                if let Some(rest) = input.strip_prefix(TextTokenTag::$token_variant.as_ref()) {
                    return Some((TextTokenTag::$token_variant, rest));
                }
            };
        }
        try_parse!(Single);
        try_parse!(Double);
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn positive() {
        macro_rules! case {
            ($input:literal -> $token:ident, $rest:literal) => {{
                eprintln!("TEST: {}", $input);
                assert_eq!(
                    TextTokenTag::parse($input),
                    Some((TextTokenTag::$token, $rest)),
                );
            }};
        }
        case!("'''" -> Single, "");
        case!("\"\"\"" -> Double, "");
        case!("'''abc" -> Single, "abc");
        case!("\"\"\"abc" -> Double, "abc");
        case!("''''" -> Single, "'");
        case!("\"\"\"\"" -> Double, "\"");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {}", $input);
                assert_eq!(TextTokenTag::parse($input), None);
            }};
        }
        case!("");
        case!("'");
        case!("\"");
        case!("''");
        case!("\"\"");
    }
}
