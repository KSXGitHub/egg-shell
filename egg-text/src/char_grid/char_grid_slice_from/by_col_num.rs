use super::CharGridSliceFrom;
use crate::{CharAt, CharCount, ColNum, ColNumOutOfBound, SliceFrom, TryIterChar};

impl<BaseGrid> CharAt<ColNum> for CharGridSliceFrom<BaseGrid, ColNum>
where
    BaseGrid: CharAt<ColNum>,
{
    type Char = BaseGrid::Char;
    type Error = BaseGrid::Error;

    fn char_at(self, col_num: ColNum) -> Result<Self::Char, Self::Error> {
        let col_num = self.start.advance_by(col_num.pred_count());
        self.grid.char_at(col_num)
    }
}

impl<BaseGrid> SliceFrom<ColNum> for CharGridSliceFrom<BaseGrid, ColNum>
where
    BaseGrid: SliceFrom<ColNum>,
{
    type Slice = BaseGrid::Slice;
    type Error = BaseGrid::Error;

    fn slice_from(self, start: ColNum) -> Result<Self::Slice, Self::Error> {
        let start = self.start.advance_by(start.pred_count());
        self.grid.slice_from(start)
    }
}

impl<BaseGrid> CharCount for CharGridSliceFrom<BaseGrid, ColNum>
where
    BaseGrid: CharCount,
{
    fn char_count(&self) -> usize {
        let total = self.grid.char_count();
        let skipped = self.start.pred_count();
        total - skipped
    }
}

/// Character iterator of [`CharGridSliceFrom<_, ColNum>`]
pub struct ColNumCharIter<GridRef>
where
    GridRef: CharAt<ColNum> + Copy,
    GridRef::Error: TryInto<ColNumOutOfBound>,
{
    grid: GridRef,
    col_num: ColNum,
}

impl<GridRef> Iterator for ColNumCharIter<GridRef>
where
    GridRef: CharAt<ColNum> + Copy,
    GridRef::Error: TryInto<ColNumOutOfBound>,
{
    type Item = Result<GridRef::Char, <GridRef::Error as TryInto<ColNumOutOfBound>>::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.grid.char_at(self.col_num).map_err(TryInto::try_into) {
            Ok(char) => Ok(char),
            Err(Ok(ColNumOutOfBound)) => return None,
            Err(Err(error)) => Err(error),
        };
        self.col_num = self.col_num.advance_by(1);
        Some(item)
    }
}

impl<BaseGridRef> TryIterChar for CharGridSliceFrom<BaseGridRef, ColNum>
where
    BaseGridRef: CharAt<ColNum> + Copy,
    BaseGridRef::Error: TryInto<ColNumOutOfBound>,
{
    type Char = BaseGridRef::Char;
    type Error = <BaseGridRef::Error as TryInto<ColNumOutOfBound>>::Error;
    type CharResultIter = ColNumCharIter<BaseGridRef>;

    fn try_iter_char(self) -> Self::CharResultIter {
        ColNumCharIter {
            grid: self.grid,
            col_num: self.start,
        }
    }
}
