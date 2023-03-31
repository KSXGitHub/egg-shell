/// Token before reprocessing.
pub enum RawToken {
    /// Single line comment.
    ///
    /// A single line comment starts with the character `#`.
    Comment,

    /// Keyword or normal identifier.
    Word,

    /// String, exotic syntax, or abnormal identifier.
    ///
    /// * String could be `"abc"`, `'abc'`, `prefix"abc"`, or `prefix'abc'`.
    /// * Exotic syntax could be `rg"[a-z]+"`, `gl"src/**/*.rs"`.
    /// * Abnormal identifier could be `id"abc"`, `id'abc'`, `idl"egg: abc"`, or `idl'egg: abc'`.
    String,
}
