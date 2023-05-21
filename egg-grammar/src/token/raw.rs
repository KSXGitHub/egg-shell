use super::{InsertWhitespaces, ParseEmbedTokenAttr, ParseEmbedTokenBody};
use derive_more::{AsMut, AsRef, Deref, DerefMut, Display};
use pipe_trait::Pipe;

/// Token to not parse.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut)]
pub struct RawToken<Content>(pub Content);

impl<'a> ParseEmbedTokenAttr<&'a str> for RawToken<&'a str> {
    fn parse(input: &'a str) -> Option<Self> {
        input.pipe(RawToken).pipe(Some)
    }
}

impl<'a> ParseEmbedTokenBody<&'a str> for RawToken<&'a str> {
    fn parse(input: &'a str) -> Option<Self> {
        input.pipe(RawToken).pipe(Some)
    }
}

impl<'a> InsertWhitespaces<&'a str> for Vec<RawToken<&'a str>> {
    fn insert_whitespaces(&mut self, ws: &'a str) -> Option<()> {
        self.push(RawToken(ws));
        Some(())
    }
}
