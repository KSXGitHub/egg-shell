use derive_more::{Display, Error};

/// Detected error when parse a [`StringToken`](super::StringToken).
///
/// Due to the interface of [`ParseMiddleToken`](crate::token::ParseMiddleToken),
/// the error shall not be emitted immediately during the tokenization process,
/// instead, it will be thrown during the AST parsing process.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum Error {
    #[display(fmt = "String is not terminated properly with a matching quote")]
    EndQuoteNotFound,
}
