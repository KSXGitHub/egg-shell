/// Resolved name from an identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Name<Namespace, Value> {
    /// Code of the target namespace or language.
    pub namespace: Namespace,
    /// Value of the identifier.
    pub value: Value,
}

/// Resolved name from an optional identifier.
///
/// Sometimes, it is intentional to leave a binding name unused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptionalName<Language, Value> {
    /// The binding can be ignored.
    Ignored(Option<Name<Language, Value>>),
    /// The binding should not be ignored.
    Named(Name<Language, Value>),
}
