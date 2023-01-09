use super::CharGridSliceFrom;
use crate::{CharAt, CharCount, CharPos, CharPosOutOfBound, SliceFrom, TryIterChar};

impl<BaseGrid> CharAt<CharPos> for CharGridSliceFrom<BaseGrid, CharPos>
where
    BaseGrid: CharAt<CharPos>,
{
    type Char = BaseGrid::Char;
    type Error = BaseGrid::Error;

    fn char_at(self, pos: CharPos) -> Result<Self::Char, Self::Error> {
        let pos = self.start.advance_by(pos.pred_count());
        self.grid.char_at(pos)
    }
}

impl<BaseGrid> SliceFrom<CharPos> for CharGridSliceFrom<BaseGrid, CharPos>
where
    BaseGrid: SliceFrom<CharPos>,
{
    type Slice = BaseGrid::Slice;
    type Error = BaseGrid::Error;

    fn slice_from(self, start: CharPos) -> Result<Self::Slice, Self::Error> {
        let start = self.start.advance_by(start.pred_count());
        self.grid.slice_from(start)
    }
}

impl<BaseGrid> CharCount for CharGridSliceFrom<BaseGrid, CharPos>
where
    BaseGrid: CharCount,
{
    fn char_count(&self) -> usize {
        let total = self.grid.char_count();
        let skipped = self.start.pred_count();
        total - skipped
    }
}

/// Character iterator of [`CharGridSliceFrom<_, CharPos>`].
pub struct CharPosCharIter<GridRef>
where
    GridRef: CharAt<CharPos> + Copy,
    GridRef::Error: TryInto<CharPosOutOfBound>,
{
    char_pos: CharPos,
    grid: GridRef,
}

impl<GridRef> Iterator for CharPosCharIter<GridRef>
where
    GridRef: CharAt<CharPos> + Copy,
    GridRef::Error: TryInto<CharPosOutOfBound>,
{
    type Item = Result<GridRef::Char, <GridRef::Error as TryInto<CharPosOutOfBound>>::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.grid.char_at(self.char_pos).map_err(TryInto::try_into) {
            Ok(char) => Ok(char),
            Err(Ok(CharPosOutOfBound)) => return None,
            Err(Err(error)) => Err(error),
        };
        self.char_pos = self.char_pos.advance_by(1);
        Some(item)
    }
}

impl<BaseGridRef> TryIterChar for CharGridSliceFrom<BaseGridRef, CharPos>
where
    BaseGridRef: CharAt<CharPos> + Copy,
    BaseGridRef::Error: TryInto<CharPosOutOfBound>,
{
    type Char = BaseGridRef::Char;
    type Error = <BaseGridRef::Error as TryInto<CharPosOutOfBound>>::Error;
    type CharResultIter = CharPosCharIter<BaseGridRef>;
    fn try_iter_char(self) -> Self::CharResultIter {
        CharPosCharIter {
            char_pos: self.start,
            grid: self.grid,
        }
    }
}
