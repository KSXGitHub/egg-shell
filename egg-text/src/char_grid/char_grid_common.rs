use super::{lazy_char_grid, CompletedCharGrid, LazyCharGrid};

/// Shared behavior of grid structs for private uses.
pub trait GridCommon {
    /// Slice of the underlying text.
    type Slice;
    /// Get a slice of the underlying text.
    fn inner_text_slice(self, start: usize, end: usize) -> Self::Slice;
}

impl<'a, CharIter: 'a> GridCommon for &'a LazyCharGrid<CharIter> {
    type Slice = lazy_char_grid::InnerTextSlice<'a, CharIter>;

    fn inner_text_slice(self, start: usize, end: usize) -> Self::Slice {
        lazy_char_grid::InnerTextSlice::new(self, start, end)
    }
}

impl<'a> GridCommon for &'a CompletedCharGrid {
    type Slice = &'a str;

    fn inner_text_slice(self, start: usize, end: usize) -> Self::Slice {
        &self.text[start..end]
    }
}
