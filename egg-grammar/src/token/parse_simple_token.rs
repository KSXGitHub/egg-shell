/// Parse a specific type of token.
pub trait ParseSimpleToken<Content>: Sized {
    /// Parse the input text into a token.
    ///
    /// **Return value:**
    /// * `None` means the token does not match.
    /// * `Some((token, rest))` means that `token` is the token and `rest` is the remaining unparsed string.
    fn parse(input: Content) -> Option<(Self, Content)>;
}
