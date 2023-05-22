use super::{EndingToken, IndentToken, MiddleToken};
use derive_more::Constructor;
use egg_ast::LnNum;

/// List of tokens from a line.
#[derive(Debug, Clone, PartialEq, Eq, Constructor)]
pub struct TokenLine<Content> {
    /// The number of the line.
    pub ln_num: LnNum,
    /// Token of the indentation at the start of the line.
    pub indent: TokenLineItem<Content, IndentToken>,
    /// List of [`MiddleToken`] after indentation.
    pub middle: Vec<TokenLineItem<Content, MiddleToken<Content>>>,
    /// Optional [`EndingToken`] at the end of the line.
    pub ending: Option<TokenLineItem<Content, EndingToken<Content>>>,
}

/// Item of [`TokenLine`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Constructor)]
pub struct TokenLineItem<SrcText, Token> {
    /// The original text that was parsed into the token.
    pub src_text: SrcText,
    /// The token that was parsed from the source text.
    pub token: Token,
}
