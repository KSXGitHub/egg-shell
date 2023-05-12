use super::EmbedToken;
use crate::token::{IndentToken, ParseEmbedTokenAttr, ParseEmbedTokenBody, ParseEmbedTokenTag};

/// The first item is not parsed yet.
type Empty = ();
/// The first item is parsed.
type Inhabited = (String, IndentToken);

/// Builder for [`EmbedToken`].
///
/// This builder takes indentation into account (unlike using `EmbedToken` directly).
#[derive(Debug, Clone)]
pub struct EmbedTokenBuilder<'header_indent, FirstBodyIndent, Tag, Attr, Body> {
    header_indent: &'header_indent IndentToken,
    first_body_indent: FirstBodyIndent,
    token: EmbedToken<Tag, Attr, Body>,
}

impl<'header_indent, FirstBodyIndent, Tag, Attr, Body>
    EmbedTokenBuilder<'header_indent, FirstBodyIndent, Tag, Attr, Body>
{
    /// Extract the [token](EmbedToken) that was built.
    pub fn finish(self) -> EmbedToken<Tag, Attr, Body> {
        self.token
    }
}

impl<'header_indent, 'input, Tag, Attr, Body>
    EmbedTokenBuilder<'header_indent, Empty, Tag, Attr, Body>
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
            first_body_indent: (),
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

impl<'header_indent, 'input, Tag, Attr, Body>
    EmbedTokenBuilder<'header_indent, Empty, Tag, Attr, Body>
where
    Body: ParseEmbedTokenBody<&'input str>,
{
    /// Parse the body's first item and indentation.
    pub fn parse_first_body_item(
        self,
        input: &'input str,
    ) -> Option<EmbedTokenBuilder<'header_indent, Inhabited, Tag, Attr, Body>> {
        let EmbedTokenBuilder {
            header_indent,
            first_body_indent: (),
            mut token,
        } = self;
        let (first_body_indent, input) = IndentToken::parse_line(input);
        if !header_indent.is_shorter_start_of(&first_body_indent) {
            return None;
        }
        token.parse_body_item(input)?;
        let first_body_indent = (first_body_indent.to_string(), first_body_indent);
        let builder = EmbedTokenBuilder {
            header_indent,
            first_body_indent,
            token,
        };
        Some(builder)
    }
}

impl<'header_indent, 'input, Tag, Attr, Body>
    EmbedTokenBuilder<'header_indent, Inhabited, Tag, Attr, Body>
where
    Body: ParseEmbedTokenBody<&'input str>,
{
    /// If the input has the same indent as the body's first indent, parse the input and add the resulting token.
    pub fn parse_next_body_item(&mut self, input: &'input str) -> Option<()> {
        let (first_body_indent, _) = &self.first_body_indent;
        let input = input.strip_prefix(first_body_indent)?;
        self.token.parse_body_item(input)
    }
}
