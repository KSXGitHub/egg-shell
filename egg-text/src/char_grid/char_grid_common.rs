use super::{lazy_char_grid, CompletedCharGrid, LazyCharGrid};

/// Shared behavior of grid structs for private uses.
pub trait GridCommon<'a> {
    /// Slice of the underlying text.
    type Slice: 'a;
    /// Get a slice of the underlying text.
    fn inner_text_slice(&'a self, start: usize, end: usize) -> Self::Slice;
}

impl<'a, CharIter: 'a> GridCommon<'a> for LazyCharGrid<CharIter> {
    type Slice = lazy_char_grid::InnerTextSlice<'a, CharIter>;

    fn inner_text_slice(&'a self, start: usize, end: usize) -> Self::Slice {
        lazy_char_grid::InnerTextSlice::new(self, start, end)
    }
}

impl<'a> GridCommon<'a> for CompletedCharGrid {
    type Slice = &'a str;

    fn inner_text_slice(&'a self, start: usize, end: usize) -> Self::Slice {
        &self.text[start..end]
    }
}
