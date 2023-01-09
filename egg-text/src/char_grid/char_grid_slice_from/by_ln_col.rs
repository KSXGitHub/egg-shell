use super::CharGridSliceFrom;
use crate::{CharAt, LineAt, LineCount, LnCol, LnColOutOfBound, LnNum, SliceFrom, TryIterChar};
use std::convert::Infallible;

impl<BaseGrid> CharAt<LnCol> for CharGridSliceFrom<BaseGrid, LnCol>
where
    BaseGrid: CharAt<LnCol>,
{
    type Char = BaseGrid::Char;
    type Error = BaseGrid::Error;

    fn char_at(self, coord: LnCol) -> Result<Self::Char, Self::Error> {
        let CharGridSliceFrom { grid, start } = self;
        let coord = match coord.line.pred_count() {
            0 => start.advance_column(coord.column.pred_count()),
            _ => coord.advance_line(start.line.pred_count()),
        };
        grid.char_at(coord)
    }
}

impl<BaseGrid> LineAt<LnNum> for CharGridSliceFrom<BaseGrid, LnCol>
where
    BaseGrid: LineAt<LnNum>,
{
    type Line = BaseGrid::Line;
    type Error = BaseGrid::Error;

    fn line_at(self, ln_num: LnNum) -> Result<Self::Line, Self::Error> {
        let ln_num = self.start.line.advance_by(ln_num.pred_count());
        self.grid.line_at(ln_num)
    }
}

impl<BaseGrid> SliceFrom<LnCol> for CharGridSliceFrom<BaseGrid, LnCol> {
    type Slice = CharGridSliceFrom<Self, LnCol>;
    type Error = Infallible;
    fn slice_from(self, start: LnCol) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl<BaseGrid> LineCount for CharGridSliceFrom<BaseGrid, LnCol>
where
    BaseGrid: LineCount,
{
    fn line_count(&self) -> usize {
        let total = self.grid.line_count();
        let skipped = self.start.column.pred_count();
        total - skipped
    }
}

/// Character iterator of [`CharGridSliceFrom<_, LnCol>`].
pub struct LnColCharIter<GridRef>
where
    GridRef: CharAt<LnCol> + Copy,
    GridRef::Error: TryInto<LnColOutOfBound>,
{
    coord: LnCol,
    grid: GridRef,
}

impl<GridRef> Iterator for LnColCharIter<GridRef>
where
    GridRef: CharAt<LnCol> + Copy,
    GridRef::Error: TryInto<LnColOutOfBound>,
{
    type Item = Result<GridRef::Char, <GridRef::Error as TryInto<LnColOutOfBound>>::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.grid.char_at(self.coord).map_err(TryInto::try_into) {
            Ok(item) => {
                self.coord = self.coord.advance_column(1);
                Some(Ok(item))
            }
            Err(Ok(LnColOutOfBound::ColumnOutOfBound)) => {
                let ln_pred_count = self.coord.line.pred_count() + 1;
                self.coord = LnCol::from_pred_counts(ln_pred_count, 0);
                self.next()
            }
            Err(Ok(LnColOutOfBound::LineOutOfBound)) => None,
            Err(Err(error)) => Some(Err(error)),
        }
    }
}

impl<BaseGridRef> TryIterChar for CharGridSliceFrom<BaseGridRef, LnCol>
where
    BaseGridRef: CharAt<LnCol> + Copy,
    BaseGridRef::Error: TryInto<LnColOutOfBound>,
{
    type Char = BaseGridRef::Char;
    type Error = <BaseGridRef::Error as TryInto<LnColOutOfBound>>::Error;
    type CharResultIter = LnColCharIter<BaseGridRef>;

    fn try_iter_char(self) -> Self::CharResultIter {
        LnColCharIter {
            coord: self.start,
            grid: self.grid,
        }
    }
}

#[cfg(test)]
mod test;
