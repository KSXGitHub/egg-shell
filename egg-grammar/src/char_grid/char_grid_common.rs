use super::LazyCharGrid;

/// Shared behavior of grid structs for private uses.
pub trait GridCommon {
    /// Get a reference to the underlying text.
    fn loaded_text(&self) -> &'_ str;
}

impl<CharIter> GridCommon for LazyCharGrid<CharIter> {
    fn loaded_text(&self) -> &'_ str {
        &self.loaded_text
    }
}
