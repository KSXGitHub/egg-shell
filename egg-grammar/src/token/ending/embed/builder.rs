use super::EmbedToken;
use crate::token::{IndentToken, ParseEmbedTokenAttr, ParseEmbedTokenBody, ParseEmbedTokenTag};

#[derive(Debug, Clone)]
pub struct EmbedTokenBuilder<'a, Tag, Attr, Body> {
    indent: &'a IndentToken,
    token: EmbedToken<Tag, Attr, Body>,
}

impl<'a, Tag, Attr, Body> EmbedTokenBuilder<'a, Tag, Attr, Body> {
    pub fn finish(self) -> EmbedToken<Tag, Attr, Body> {
        self.token
    }
}

impl<'indent, 'input, Tag, Attr, Body> EmbedTokenBuilder<'indent, Tag, Attr, Body>
where
    Tag: ParseEmbedTokenTag<&'input str>,
    Attr: ParseEmbedTokenAttr<&'input str>,
{
    pub fn start_parsing(
        indent: &'indent IndentToken,
        input: &'input str,
        body: Vec<Body>,
    ) -> Option<Self> {
        let token = EmbedToken::<Tag, Attr, Body>::start_parsing(input, body)?;
        let builder = EmbedTokenBuilder { indent, token };
        Some(builder)
    }

    pub fn new(indent: &'indent IndentToken, input: &'input str) -> Option<Self> {
        Self::start_parsing(indent, input, Vec::new())
    }

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
    pub fn parse_body_item(&mut self, input: &'input str) -> Option<()> {
        let (input_indent, input) = IndentToken::parse_line(input);
        if self.indent.is_start_of(&input_indent) {
            self.token.parse_body_item(input)
        } else {
            None
        }
    }
}
