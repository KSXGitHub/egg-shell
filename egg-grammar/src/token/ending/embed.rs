use crate::token::{ParseEmbedTokenAttr, ParseEmbedTokenBody, ParseEmbedTokenTag};

pub mod doc;
pub mod text;

pub use doc::DocToken;
pub use text::TextToken;

/// Token for a chunk of embedded lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbedToken<Tag, Attr, Body> {
    pub header: (Tag, Attr),
    pub body: Vec<Body>,
}

impl<'a, Tag, Attr, Body> EmbedToken<Tag, Attr, Body>
where
    Tag: ParseEmbedTokenTag<&'a str>,
    Attr: ParseEmbedTokenAttr<&'a str>,
{
    /// Parse the header of the embed token.
    pub fn parse_header(input: &'a str) -> Option<(Tag, Attr)> {
        let (tag, input) = Tag::parse(input)?;
        let attr = Attr::parse(input)?;
        Some((tag, attr))
    }

    /// Start the parsing process with a header and an existing body.
    ///
    /// The existing body is typically an empty `Vec` constructed by `Vec::new` or `Vec::with_capacity`.
    pub fn start_parsing(input: &'a str, body: Vec<Body>) -> Option<Self> {
        let header = Self::parse_header(input)?;
        let token = EmbedToken { header, body };
        Some(token)
    }
}

impl<'a, Tag, Attr, Body> EmbedToken<Tag, Attr, Body>
where
    Body: ParseEmbedTokenBody<&'a str>,
{
    /// Parse an item and add it to the body.
    pub fn parse_body_item(&mut self, input: &'a str) -> Option<&'_ mut Self> {
        let item = Body::parse(input)?;
        self.body.push(item);
        Some(self)
    }
}
