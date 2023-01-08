use super::LazyCharGrid;
use crate::{char_grid::CharGridSliceFrom, CharPos, LnCol, LnNum, SliceFrom};
use std::convert::Infallible;

impl<'a, SrcIter: 'a> SliceFrom<LnNum> for &'a LazyCharGrid<SrcIter> {
    type Slice = CharGridSliceFrom<Self, LnNum>;
    type Error = Infallible;
    fn slice_from(self, start: LnNum) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl<'a, SrcIter: 'a> SliceFrom<LnCol> for &'a LazyCharGrid<SrcIter> {
    type Slice = CharGridSliceFrom<Self, LnCol>;
    type Error = Infallible;
    fn slice_from(self, start: LnCol) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl<'a, SrcIter: 'a> SliceFrom<CharPos> for &'a LazyCharGrid<SrcIter> {
    type Slice = CharGridSliceFrom<Self, CharPos>;
    type Error = Infallible;
    fn slice_from(self, start: CharPos) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}
