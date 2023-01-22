/// Line definition.
#[derive(Debug, Clone, Copy)]
pub struct PartiallyClonedCharGridLineDef {
    /// Starting character offset in string.
    pub start_offset: usize,
    /// Size of the line in bytes.
    pub size: usize,
    /// Starting character index in character list.
    pub start_index: usize,
    /// Number of characters in the line.
    pub char_count: usize,
}
