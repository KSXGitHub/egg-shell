use super::{BracketToken, NumberToken, OperatorToken, StringToken, WhitespaceToken, WordToken};
use crate::token::ParseMiddleToken;
use derive_more::{From, TryInto};

/// Token in the middle of the line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, From, TryInto)]
#[non_exhaustive]
pub enum MiddleToken<Content> {
    /// Single character of Space or Tab.
    Whitespace(WhitespaceToken),

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
        try_parse!(WhitespaceToken);
        try_parse!(StringToken);
        try_parse!(WordToken);
        try_parse!(NumberToken);
        try_parse!(BracketToken);
        try_parse!(OperatorToken);
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! def_macro {
        ($name:ident -> $token_variant:ident) => {
            /// This macro will not test the exact value of the token, only the top-level variant.
            macro_rules! $name {
                ($input:literal -> $rest:literal) => {{
                    eprintln!();
                    eprintln!("TEST: {:?}", $input);
                    let (token, rest) = MiddleToken::parse($input).expect("Parse failure");
                    dbg!(token);
                    assert!(matches!(token, MiddleToken::$token_variant(_)));
                    dbg!(rest);
                    assert_eq!(rest, $rest);
                }};
            }
        };
    }

    #[test]
    fn whitespace() {
        def_macro!(case -> Whitespace);
        case!(" " -> "");
        case!("\t" -> "");
        case!("  " -> " ");
        case!("\t\t" -> "\t");
        case!(" abc" -> "abc");
        case!("\tabc" -> "abc");
    }

    #[test]
    fn string() {
        def_macro!(case -> String);
        case!("''" -> "");
        case!("'abc def ghi'" -> "");
        case!(r#""abc def ghi""# -> "");
        case!("1foo-bar'abc def'" -> "");
        case!(r"prefix'abc def \' ghi\n\t'suffix++' jkl mno'" -> "++' jkl mno'");
        case!("GalaxyBrainGigachadWarStratagem'三十六計，走為上計'逃げるんだよ" -> "逃げるんだよ");
        case!("'" -> "");
        case!("\"" -> "");
        case!("prefix'abc def ghi" -> "");
    }

    #[test]
    fn word() {
        def_macro!(case -> Word);
        case!("print('hello world')" -> "('hello world')");
        case!("if a + b == c then" -> " a + b == c then");
    }

    #[test]
    fn number() {
        def_macro!(case -> Number);
        case!("123_456-789" -> "-789");
        case!("123.45f64+789" -> "+789");
        case!("123.45.rest" -> ".rest");
        case!("0x123ABCi32 rest" -> " rest");
    }

    #[test]
    fn bracket() {
        def_macro!(case -> Bracket);
        case!("(abc + 123)" -> "abc + 123)");
        case!("[0, 1, 2, 3]" -> "0, 1, 2, 3]");
        case!("{ a: 0, b: 1 }" -> " a: 0, b: 1 }");
        case!(")" -> "");
        case!("]" -> "");
        case!("}" -> "");
    }

    #[test]
    fn operator() {
        def_macro!(case -> Operator);
        case!("+=123" -> "123");
        case!("+#abc" -> "abc");
        case!("+ #abc" -> " #abc");
        case!("$interpolation" -> "interpolation");
        case!("@attribute" -> "attribute");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(MiddleToken::parse($input), None);
            }};
        }

        case!("");
        case!("# this is a comment");
    }
}
