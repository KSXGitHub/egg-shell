use super::EmbedToken;
use crate::token::{
    IndentToken, InsertWhitespaces, ParseEmbedTokenAttr, ParseEmbedTokenBody, ParseEmbedTokenTag,
};
use pipe_trait::Pipe;

/// Builder for [`EmbedToken`].
///
/// This builder takes indentation into account (unlike using `EmbedToken` directly).
#[derive(Debug, Clone)]
pub struct EmbedTokenBuilder<'header_indent, Tag, Attr, Body> {
    header_indent: &'header_indent IndentToken,
    first_body_indent: Option<String>,
    token: EmbedToken<Tag, Attr, Body>,
}

impl<'header_indent, Tag, Attr, Body> EmbedTokenBuilder<'header_indent, Tag, Attr, Body> {
    /// Extract the [token](EmbedToken) that was built.
    pub fn finish(self) -> EmbedToken<Tag, Attr, Body> {
        self.token
    }
}

impl<'header_indent, 'input, Tag, Attr, Body> EmbedTokenBuilder<'header_indent, Tag, Attr, Body>
where
    Tag: ParseEmbedTokenTag<&'input str>,
    Attr: ParseEmbedTokenAttr<&'input str>,
{
    /// Start the parsing process with a header and an existing body.
    ///
    /// The existing body is typically an empty `Vec` constructed by `Vec::new` or `Vec::with_capacity`.
    pub fn start_parsing(
        header_indent: &'header_indent IndentToken,
        input: &'input str,
        body: Vec<Body>,
    ) -> Option<Self> {
        let token = EmbedToken::<Tag, Attr, Body>::start_parsing(input, body)?;
        let builder = EmbedTokenBuilder {
            header_indent,
            first_body_indent: None,
            token,
        };
        Some(builder)
    }

    /// Start the parsing process with an empty body.
    pub fn new(indent: &'header_indent IndentToken, input: &'input str) -> Option<Self> {
        Self::start_parsing(indent, input, Vec::new())
    }

    /// Start the parsing process with an empty body with a specified capacity.
    pub fn with_capacity(
        indent: &'header_indent IndentToken,
        input: &'input str,
        capacity: usize,
    ) -> Option<Self> {
        Self::start_parsing(indent, input, Vec::with_capacity(capacity))
    }
}

impl<'header_indent, 'input, Tag, Attr, Body> EmbedTokenBuilder<'header_indent, Tag, Attr, Body>
where
    Body: ParseEmbedTokenBody<&'input str>,
    Vec<Body>: InsertWhitespaces<&'input str>,
{
    pub fn parse_body_item(&mut self, input: &'input str) -> Option<()> {
        if input.trim().is_empty() {
            return self.token.insert_body_ws(input);
        }

        if let Some(first_body_indent) = &self.first_body_indent {
            let input = input.strip_prefix(first_body_indent)?;
            return self.token.parse_body_item(input);
        }

        let (first_body_indent, input) = IndentToken::parse(input);
        if !self.header_indent.is_shorter_start_of(&first_body_indent) {
            return None;
        }
        self.token.parse_body_item(input)?;
        self.first_body_indent = first_body_indent.to_string().pipe(Some);
        Some(())
    }
}
