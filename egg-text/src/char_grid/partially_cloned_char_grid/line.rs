/// Line definition.
#[derive(Debug, Clone, Copy)]
pub struct PartiallyClonedCharGridLineDef {
    /// Starting character index in character list.
    pub start_index: usize,
    /// Number of characters in the line.
    pub char_count: usize,
}
