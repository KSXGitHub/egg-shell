use super::{EndingToken, IndentToken, InvalidToken, MiddleToken};
use derive_more::Constructor;

/// Either a [`MiddleToken`] or an [`InvalidToken`].
type MiddleTokenResult<Content> = Result<MiddleToken<Content>, InvalidToken>;

/// [`TokenLineItem`] of an [`EndingToken`].
type EndingTokenItem<Content> = TokenLineItem<(Content, Vec<Content>), EndingToken<Content>>;

/// List of tokens from a line.
#[derive(Debug, Clone, PartialEq, Eq, Constructor)]
pub struct TokenLine<Content> {
    /// The content of the line.
    pub ln_text: Content,
    /// Token of the indentation at the start of the line.
    pub indent: TokenLineItem<Content, IndentToken>,
    /// List of [`MiddleToken`] after indentation.
    pub middle: Vec<TokenLineItem<Content, MiddleTokenResult<Content>>>,
    /// Optional [`EndingToken`] at the end of the line.
    pub ending: Option<EndingTokenItem<Content>>,
}

/// Item of [`TokenLine`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Constructor)]
pub struct TokenLineItem<SrcText, Token> {
    /// Offset of the text from the start of the line.
    pub offset: usize,
    /// The original text that was parsed into the token.
    pub src_text: SrcText,
    /// The token that was parsed from the source text.
    pub token: Token,
}
