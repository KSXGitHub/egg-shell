use crate::{char_grid::CharGridSliceFrom, CharPos, CompletedCharGrid, LnCol, LnNum, SliceFrom};
use std::convert::Infallible;

impl<'a> SliceFrom<LnNum> for &'a CompletedCharGrid {
    type Slice = CharGridSliceFrom<Self, LnNum>;
    type Error = Infallible;
    fn slice_from(self, start: LnNum) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl<'a> SliceFrom<LnCol> for &'a CompletedCharGrid {
    type Slice = CharGridSliceFrom<Self, LnCol>;
    type Error = Infallible;
    fn slice_from(self, start: LnCol) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl<'a> SliceFrom<CharPos> for &'a CompletedCharGrid {
    type Slice = CharGridSliceFrom<Self, CharPos>;
    type Error = Infallible;
    fn slice_from(self, start: CharPos) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}
