/// Parse a middle token.
pub trait ParseMiddleToken<Content>: Sized {
    /// Parse the input text into a token.
    ///
    /// **Return value:**
    /// * `None` means the token does not match.
    /// * `Some((token, rest))` means that `token` is the token and `rest` is the remaining unparsed string.
    fn parse(input: Content) -> Option<(Self, Content)>;
}

/// Parse a tag of an embedded token.
pub trait ParseEmbedTokenTag<Content>: Sized {
    /// Parse the input text into a tag of an embedded token.
    ///
    /// **Return value:**
    /// * `None` means the token does not match.
    /// * `Some((tag, rest))` means that `tag` is the tag and `rest` is the remaining unparsed string.
    fn parse(input: Content) -> Option<(Self, Content)>;
}

/// Parse attribute of an embedded token.
pub trait ParseEmbedTokenAttr<Content>: Sized {
    /// Parse the input text into an attribute of an embedded token.
    ///
    /// **Return value:**
    /// * `None` means the token does not match.
    /// * `Some(attr)` means that `attr` is the attribute.
    fn parse(input: Content) -> Option<Self>;
}

/// Parse a line of a body of an embedded token.
pub trait ParseEmbedTokenBody<Content>: Sized {
    /// Parse the input text into a line of a body of an embedded token.
    ///
    /// **Return value:**
    /// * `None` means the token does not match.
    /// * `Some(body)` means that `body` is the body line.
    fn parse(input: Content) -> Option<Self>;
}

/// Add a string of only whitespaces.
pub trait InsertWhitespaces<Whitespace> {
    /// Parse a string of whitespaces into a token and add it to `self`.
    ///
    /// **Return value:**
    /// * `None` means the whitespace is rejected.
    /// * `Some(())` means that the whitespace is added to `self`.
    fn insert_whitespaces(&mut self, ws: Whitespace) -> Option<()>;
}
