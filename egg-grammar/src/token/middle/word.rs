use crate::keyword::Keyword;
use derive_more::{From, TryInto};

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

    /// Get the reference to the internal string.
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
