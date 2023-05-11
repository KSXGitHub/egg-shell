use super::EmbedToken;
use crate::token::IndentToken;

#[derive(Debug, Clone)]
pub struct EmbedTokenBuilder<'a, Tag, Attr, Body> {
    header_indent: &'a IndentToken,
    token: EmbedToken<Tag, Attr, Body>,
}
