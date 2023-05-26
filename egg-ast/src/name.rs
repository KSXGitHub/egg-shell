/// Resolved name from an identifier.
///
/// Multiple syntaxes/languages require multiple different naming conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Name<Language, Value> {
    /// Code of the target language or syntax flavor.
    pub language: Language,
    /// Value of the identifier.
    pub value: Value,
}
