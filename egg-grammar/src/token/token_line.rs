use super::{EndingToken, IndentToken, MiddleToken};
use egg_ast::{LnCol, LnNum};

/// List of tokens from a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenLine<Content> {
    /// The number of the line.
    pub ln_num: LnNum,
    /// Token of the indentation at the start of the line.
    pub indent: (LnCol, IndentToken),
    /// List of [`MiddleToken`] after indentation.
    pub middle: Vec<(LnCol, MiddleToken<Content>)>,
    /// Optional [`EndingToken`] at the end of the line.
    pub ending: (LnCol, Option<EndingToken<Content>>),
}
