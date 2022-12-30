use super::{LazyCharGrid, LazyCharGridData};
use parking_lot::RwLockReadGuard;
use std::fmt::{self, Debug, Display, Formatter};

/// Slice of inner text of [`LazyCharGrid`].
pub struct InnerTextSlice<'a, CharIter> {
    data: RwLockReadGuard<'a, LazyCharGridData<CharIter>>,
    start: usize,
    end: usize,
}

impl<'a, CharIter> InnerTextSlice<'a, CharIter> {
    /// Create a slice.
    pub fn new(grid: &'a LazyCharGrid<CharIter>, start: usize, end: usize) -> Self {
        let data = grid.data();
        InnerTextSlice { data, start, end }
    }

    /// Perform an action on the internal string.
    pub fn run<Act, Return>(&self, act: Act) -> Return
    where
        Act: FnOnce(&str) -> Return,
    {
        act(&self.data.loaded_text[self.start..self.end])
    }
}

impl<'a, CharIter> Display for InnerTextSlice<'a, CharIter> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.run(|text| write!(f, "{text}"))
    }
}

impl<'a, CharIter> Debug for InnerTextSlice<'a, CharIter> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.run(|text| write!(f, "InnerTextSlice {text:?}"))
    }
}

impl<'a, CharIter> PartialEq<str> for InnerTextSlice<'a, CharIter> {
    fn eq(&self, other: &str) -> bool {
        self.run(|text| text == other)
    }
}

impl<'a, CharIter> PartialEq<Self> for InnerTextSlice<'a, CharIter> {
    fn eq(&self, other: &Self) -> bool {
        self.run(|a| other.run(|b| a == b))
    }
}

impl<'a, CharIter> Eq for InnerTextSlice<'a, CharIter> {}
