use super::EmbedToken;
use crate::token::{IndentToken, ParseEmbedTokenAttr, ParseEmbedTokenBody, ParseEmbedTokenTag};

/// Builder for [`EmbedToken`].
///
/// This builder takes indentation into account (unlike using `EmbedToken` directly).
#[derive(Debug, Clone)]
pub struct EmbedTokenBuilder<'a, Tag, Attr, Body> {
    indent: &'a IndentToken,
    token: EmbedToken<Tag, Attr, Body>,
}

impl<'a, Tag, Attr, Body> EmbedTokenBuilder<'a, Tag, Attr, Body> {
    /// Extract the [token](EmbedToken) that was built.
    pub fn finish(self) -> EmbedToken<Tag, Attr, Body> {
        self.token
    }
}

impl<'indent, 'input, Tag, Attr, Body> EmbedTokenBuilder<'indent, Tag, Attr, Body>
where
    Tag: ParseEmbedTokenTag<&'input str>,
    Attr: ParseEmbedTokenAttr<&'input str>,
{
    /// Start the parsing process with a header and an existing body.
    ///
    /// The existing body is typically an empty `Vec` constructed by `Vec::new` or `Vec::with_capacity`.
    pub fn start_parsing(
        indent: &'indent IndentToken,
        input: &'input str,
        body: Vec<Body>,
    ) -> Option<Self> {
        let token = EmbedToken::<Tag, Attr, Body>::start_parsing(input, body)?;
        let builder = EmbedTokenBuilder { indent, token };
        Some(builder)
    }

    /// Start the parsing process with an empty body.
    pub fn new(indent: &'indent IndentToken, input: &'input str) -> Option<Self> {
        Self::start_parsing(indent, input, Vec::new())
    }

    /// Start the parsing process with an empty body with a specified capacity.
    pub fn with_capacity(
        indent: &'indent IndentToken,
        input: &'input str,
        capacity: usize,
    ) -> Option<Self> {
        Self::start_parsing(indent, input, Vec::with_capacity(capacity))
    }
}

impl<'indent, 'input, Tag, Attr, Body> EmbedTokenBuilder<'indent, Tag, Attr, Body>
where
    Body: ParseEmbedTokenBody<&'input str>,
{
    /// Parse an indent, check the indent, then parse and add a body line if the indent matches.
    pub fn parse_body_item(&mut self, input: &'input str) -> Option<()> {
        let (input_indent, input) = IndentToken::parse_line(input);
        if self.indent.is_start_of(&input_indent) {
            self.token.parse_body_item(input)
        } else {
            None
        }
    }
}
