/// Token before reprocessing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RawToken<Content> {
    /// Single line comment.
    ///
    /// A single line comment starts with the character `#`.
    Comment(Content),

    /// Keyword or normal identifier.
    Word(Content),

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
    String {
        prefix: Option<Content>,
        suffix: Option<Content>,
        main_content: Content,
        quote_type: StringQuoteType,
    },

    /// Multi-line string, documentation, or embedded code.
    Embed(Vec<Content>),

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
    Number(Content),

    /// Round bracket, square bracket, curly bracket, open or closed.
    Bracket(Content),

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
    Operator(Content),
}

/// Quote type of [`RawToken::String`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringQuoteType {
    /// Single quotes (`'`) were used to wrap the string content.
    Single,
    /// Double quotes (`"`) were used to wrap the string content.
    Double,
}
