/// Parse a specific type of token.
pub trait ParseToken: Sized {
    /// Type of input text and remaining text
    type Content;
    /// Parse the input text into a token
    fn parse(input: Self::Content) -> Option<(Self, Self::Content)>;
}
