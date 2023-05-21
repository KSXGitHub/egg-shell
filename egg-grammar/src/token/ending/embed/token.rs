use super::EmbedTokenBuilder;
use crate::token::{
    IndentToken, InsertWhitespaces, ParseEmbedTokenAttr, ParseEmbedTokenBody, ParseEmbedTokenTag,
};

/// Token for a chunk of embedded lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbedToken<Tag, Attr, Body> {
    pub header: (Tag, Attr),
    pub body: Vec<Body>,
}

impl<'input, Tag, Attr, Body> EmbedToken<Tag, Attr, Body>
where
    Tag: ParseEmbedTokenTag<&'input str>,
    Attr: ParseEmbedTokenAttr<&'input str>,
    Body: ParseEmbedTokenBody<&'input str>,
    Vec<Body>: InsertWhitespaces<&'input str>,
{
    /// Build an [`EmbedToken`] from start to finish.
    pub(crate) fn build<'header_indent>(
        header_indent: &'header_indent IndentToken,
        header_text: &'input str,
        next_line: impl FnMut() -> Option<&'input str>,
    ) -> Option<Self> {
        EmbedTokenBuilder::build(header_indent, header_text, next_line)
    }
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

    /// Start the parsing process with an empty body.
    pub fn new(input: &'a str) -> Option<Self> {
        Self::start_parsing(input, Vec::new())
    }

    /// Start the parsing process with an empty body with a specified capacity.
    pub fn with_capacity(input: &'a str, capacity: usize) -> Option<Self> {
        Self::start_parsing(input, Vec::with_capacity(capacity))
    }
}

impl<'a, Tag, Attr, Body> EmbedToken<Tag, Attr, Body>
where
    Body: ParseEmbedTokenBody<&'a str>,
{
    /// Parse an item and add it to the body.
    pub fn parse_body_item(&mut self, input: &'a str) -> Option<()> {
        let item = Body::parse(input)?;
        self.body.push(item);
        Some(())
    }
}

impl<'a, Tag, Attr, Body> EmbedToken<Tag, Attr, Body>
where
    Vec<Body>: InsertWhitespaces<&'a str>,
{
    /// Parse a string of whitespaces and add it to the body.
    pub fn insert_body_ws(&mut self, ws: &'a str) -> Option<()> {
        self.body.insert_whitespaces(ws)
    }
}
