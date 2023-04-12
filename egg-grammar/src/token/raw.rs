/// Token before reprocessing.
#[non_exhaustive]
pub enum RawToken {
    /// Single line comment.
    ///
    /// A single line comment starts with the character `#`.
    Comment,

    /// Keyword or normal identifier.
    Word,

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
    String,

    /// Multi-line string, documentation, or embedded code.
    Embed,

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
    Number,

    /// Round bracket, square bracket, curly bracket, open or closed.
    Bracket,

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
    Operator,
}
