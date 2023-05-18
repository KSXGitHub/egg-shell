use crate::{keyword::Keyword, token::ParseMiddleToken};
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
    is_word_tail(char) || matches!(char, '-')
}

const fn is_word_tail(char: &char) -> bool {
    is_word_head(char) || matches!(char, '0'..='9')
}

fn parse_word(input: &str) -> (&'_ str, &'_ str) {
    let mut iter = input.chars();

    let Some(first_char) = iter.next() else {
        return ("", input);
    };
    if !is_word_head(&first_char) {
        return ("", input);
    }

    let first_char_len = 1; // because it is an ascii character.
    debug_assert_eq!(first_char_len, first_char.len_utf8());
    let tail_size = iter.take_while(is_word_body).count(); // ascii char has len_utf8 = 1
    let end_offset = first_char_len + tail_size;

    let word = &input[..end_offset];
    let last_char = word.chars().next_back().expect("word is not empty");

    if is_word_tail(&last_char) {
        let rest = &input[end_offset..];
        (word, rest)
    } else {
        let end_offset = end_offset - 1; // it's ascii, no needs to worry about panic
        let word = &input[..end_offset];
        let rest = &input[end_offset..];
        (word, rest)
    }
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
