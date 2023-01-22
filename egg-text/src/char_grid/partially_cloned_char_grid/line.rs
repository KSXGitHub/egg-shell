/// Line definition.
#[derive(Debug, Clone, Copy)]
pub struct PartiallyClonedCharGridEol {
    /// Starting character offset in string.
    pub offset: usize,
    /// Starting character index in character list.
    pub index: usize,
}
