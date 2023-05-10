pub mod documentation_token;
pub mod text_token;

pub use documentation_token::DocumentationToken;
pub use text_token::TextToken;

/// Token for a chunk of embedded lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbedToken<Tag, Attr, Body> {
    pub header: (Tag, Attr),
    pub body: Vec<Body>,
}
