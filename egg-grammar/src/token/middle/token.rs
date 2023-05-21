use super::{BracketToken, NumberToken, OperatorToken, StringToken, WordToken};
use crate::token::ParseMiddleToken;
use derive_more::{From, TryInto};

/// Token in the middle of the line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, From, TryInto)]
#[non_exhaustive]
pub enum MiddleToken<Content> {
    /// String, exotic syntax, or abnormal identifier.
    ///
    /// **Including**
    ///
    /// * String: `"abc"`, `'abc'`, `prefix"abc"`, `prefix'abc'`, etc.
    /// * Exotic syntax: `rg"[a-z]+"`, `gl"src/**/*.rs"`, etc.
    /// * Abnormal identifier: `id"abc"`, `id'abc'`, `idl"egg: abc"`, `idl'egg: abc'`, etc.
    ///
    /// **Excluding**
    ///
    /// * Multi-line string.
    String(StringToken<Content>),

    /// Keyword or normal identifier.
    Word(WordToken<Content>),

    /// Number.
    ///
    /// **Including**
    ///
    /// * Positive decimal integer: `123`, `123i32`, `123_456_789`, etc.
    /// * Positive hexadecimal integer: `0x1234ABCD`, `0x1234abcd`, `0x1234ABCD_i32`, etc.
    /// * Positive floating point: `123.45`, `123.45f`, `123.45f32`, etc.
    ///
    /// **Excluding**
    ///
    /// * Negative numbers. Composing `-` and number instead.
    /// * `nan` and `inf`. They are keywords.
    Number(NumberToken<Content>),

    /// Round bracket, square bracket, curly bracket, open or closed.
    Bracket(BracketToken),

    /// A sequence of special characters.
    ///
    /// **Including**
    ///
    /// * Expression operator: `+`, `-`, `*`, `/`, `<`, `<=`, `>`, `>=`, `==`, `!=`, etc.
    /// * Path separator (`::`).
    /// * Dot notation (`.`).
    /// * Type specifier (`:`).
    /// * Binding or assignment (`=`).
    /// * etc.
    Operator(OperatorToken<Content>),
}

impl<'a> ParseMiddleToken<&'a str> for MiddleToken<&'a str> {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        macro_rules! try_parse {
            ($token_type:ident) => {
                if let Some((token, rest)) = $token_type::parse(input) {
                    return Some((MiddleToken::from(token), rest));
                }
            };
        }
        try_parse!(StringToken);
        try_parse!(WordToken);
        try_parse!(NumberToken);
        try_parse!(BracketToken);
        try_parse!(OperatorToken);
        None
    }
}
