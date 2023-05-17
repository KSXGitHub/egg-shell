mod error;
mod quote;

pub use error::*;
pub use quote::*;

/// String-like token.
///
/// **Structure:**
/// `[prefix] <quote> <body> <quote> [suffix]`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringToken<Content> {
    pub prefix: Option<Content>,
    pub suffix: Option<Content>,
    pub body: Content,
    pub quote: Quote,
    pub error: Error,
}
