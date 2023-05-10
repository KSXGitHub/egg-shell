pub mod documentation_token;
pub mod string_token;

pub use documentation_token::DocumentationToken;
pub use string_token::StringToken;

/// Token for a chunk of embedded lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbedToken<Tag, Attr, Body> {
    pub header: (Tag, Attr),
    pub body: Vec<Body>,
}
