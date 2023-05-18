use crate::{keyword::Keyword, token::ParseMiddleToken};
use derive_more::{From, TryInto};
use egg_common_utils::split_hbt_ascii;

/// Token of an identifier or a keyword.
#[derive(Debug, Clone, Copy, PartialEq, Eq, From, TryInto)]
pub enum WordToken<Content> {
    /// Content of the token is a regular identifier.
    #[from(ignore)]
    #[try_into(ignore)]
    Identifier(Content),
    /// Content of the token is a keyword or a reserved word.
    Keyword(Keyword),
}

impl<Content> WordToken<Content> {
    /// Check if the word is an identifier.
    pub fn is_identifier(self) -> bool {
        matches!(self, WordToken::Identifier(_))
    }

    /// Check if the word is a keyword.
    pub fn is_keyword(self) -> bool {
        matches!(self, WordToken::Keyword(_))
    }
}

impl<Content> WordToken<Content>
where
    Content: AsRef<str>,
{
    /// Convert a string to a word token.
    ///
    /// **Note:** If `Content` is [`Keyword`], this method is suboptimal, please use
    /// [`WordToken::Keyword`], [`From<Keyword>::from`], or [`Into<WordToken>::into`]
    /// instead.
    pub fn from_any_str(content: Content) -> Self {
        match Keyword::try_from(content.as_ref()) {
            Ok(keyword) => WordToken::Keyword(keyword),
            Err(_) => WordToken::Identifier(content),
        }
    }

    /// Get reference to the internal string.
    pub fn as_str(&self) -> &'_ str {
        match self {
            WordToken::Identifier(identifier) => identifier.as_ref(),
            WordToken::Keyword(keyword) => keyword.as_ref(),
        }
    }
}

// Q: Why not generic over `T: AsRef<str>`?
// A: `Keyword` also implements `AsRef<str>` but the algorithm below is not optimal,
//    `T: AsRef<str>` would have misled the user into mistakenly use suboptimal `From`.
impl<'a> From<&'a str> for WordToken<&'a str> {
    fn from(input: &'a str) -> Self {
        WordToken::from_any_str(input)
    }
}

impl<Content> AsRef<str> for WordToken<Content>
where
    Content: AsRef<str>,
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

const fn is_word_head(char: &char) -> bool {
    matches!(char, 'a'..='z' | 'A'..='Z' | '_')
}

const fn is_word_body(char: &char) -> bool {
    matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-')
}

const fn is_word_tail(char: &char) -> bool {
    matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

fn parse_word(input: &str) -> (&'_ str, &'_ str) {
    split_hbt_ascii(input, is_word_head, is_word_body, is_word_tail)
}

impl<'a> ParseMiddleToken<&'a str> for WordToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (word, rest) = parse_word(input);
        if word.is_empty() {
            return None;
        }
        let token = WordToken::from(word);
        Some((token, rest))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn positive() {
        use Keyword::*;
        let id = WordToken::Identifier;
        let kw = WordToken::Keyword;

        macro_rules! case {
            ($input:literal -> $token:expr, $rest:literal) => {{
                eprintln!("CASE: {:?}", $input);
                assert_eq!(WordToken::parse($input), Some(($token, $rest)));
            }};
        }

        case!("if a + b == c then" -> kw(If), " a + b == c then");
        case!("return true" -> kw(Return), " true");
        case!("return-123" -> id("return-123"), "");
        case!("return.123" -> kw(Return), ".123");
        case!("print('hello world')" -> id("print"), "('hello world')");
        case!("a + b" -> id("a"), " + b");
        case!("abc123-def-" -> id("abc123-def"), "-");
        case!("abc123_def_" -> id("abc123_def_"), "");
        case!("_abc def" -> id("_abc"), " def");
        case!("u32 -> u32" -> kw(U32), " -> u32");
        case!("abcđef" -> id("abc"), "đef");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("CASE: {:?}", $input);
                assert_eq!(WordToken::parse($input), None);
            }};
        }

        case!("");
        case!("3a");
        case!("-abc");
        case!("âbc");
    }
}
