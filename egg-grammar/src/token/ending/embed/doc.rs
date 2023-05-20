use crate::token::{ParseEmbedTokenTag, ParseMiddleToken, RawToken, WordToken};
use derive_more::{From, Into};
use pipe_trait::Pipe;

/// Token for chunk of documentation lines.
pub type DocToken<Content> =
    super::EmbedToken<DocTokenTag<Content>, RawToken<Content>, RawToken<Content>>;

/// Tag of [`DocToken`].
///
/// **Structure:** `@@[name]`
#[derive(Debug, Clone, Copy, PartialEq, Eq, From, Into)]
pub struct DocTokenTag<Content>(pub Option<WordToken<Content>>);

impl<Content> From<WordToken<Content>> for DocTokenTag<Content> {
    fn from(token: WordToken<Content>) -> Self {
        token.pipe(Some).into()
    }
}

impl<'a> From<&'a str> for DocTokenTag<&'a str> {
    fn from(name: &'a str) -> Self {
        name.pipe(WordToken::from).into()
    }
}

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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn positive() {
        macro_rules! case {
            ($input:literal -> $token:expr, $rest:literal) => {{
                eprintln!("TEST: {:?}", $input);
                let token = $token.into();
                assert_eq!(DocTokenTag::parse($input), Some((token, $rest)));
            }};
        }

        case!("@@" -> None, "");
        case!("@@foo" -> "foo", "");
        case!("@@. abcdef" -> None, ". abcdef");
        case!("@@ Nothing to see here" -> None, " Nothing to see here");
        case!("@@desc Description of an item" -> "desc", " Description of an item");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(DocTokenTag::parse($input), None);
            }};
        }

        case!("");
        case!("@desc Description of an item");
        case!("abcdef");
    }
}
