/// Parse a specific type of token.
pub trait ParseSimpleToken<Content>: Sized {
    /// Parse the input text into a token
    fn parse(input: Content) -> Option<(Self, Content)>;
}
