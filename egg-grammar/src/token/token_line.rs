use super::{EndingToken, IndentToken, InvalidToken, MiddleToken};
use derive_more::Constructor;
use std::iter::once;

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

impl<Content> TokenLine<Content> {
    /// Iterate over [`TokenLine::ln_text`] and [`TokenLine::ending_body_ln_text`].
    pub fn all_ln_text(&self) -> impl Iterator<Item = &'_ Content> + '_ {
        once(&self.ln_text).chain(self.ending_body_ln_text())
    }

    /// Iterate over all lines that were parsed into the body part of the ending token
    /// (if there is an ending token).
    pub fn ending_body_ln_text(&self) -> impl Iterator<Item = &'_ Content> + '_ {
        self.ending
            .iter()
            .flat_map(|item| item.body_ln_text().iter())
    }
}

/// Item of [`TokenLine`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Constructor)]
pub struct TokenLineItem<SrcText, Token> {
    /// The original text that was parsed into the token.
    pub src_text: SrcText,
    /// The token that was parsed from the source text.
    pub token: Token,
}

impl<Content> EndingTokenItem<Content> {
    /// The original text that was parsed into the body part of the token.
    pub fn body_ln_text(&self) -> &'_ Vec<Content> {
        let (_, lines) = &self.src_text;
        lines
    }
}
