use crate::{
    CharAt, CharCount, CharPos, CharPosOutOfBound, ColNum, LineAt, LineCount, LnCol, LnNum,
    SliceFrom, TryIterChar,
};
use std::convert::Infallible;

/// Create a slice of char grid from a start coordinate.
///
/// The resulting slice includes the start coordinate.
#[derive(Debug, Clone, Copy)]
pub struct CharGridSliceFrom<BaseGrid, Coord> {
    /// Reference grid.
    pub grid: BaseGrid,
    /// Start coordinate.
    pub start: Coord,
}

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

impl<BaseGrid> CharAt<LnCol> for CharGridSliceFrom<BaseGrid, LnNum>
where
    BaseGrid: CharAt<LnCol>,
{
    type Char = BaseGrid::Char;
    type Error = BaseGrid::Error;

    fn char_at(self, coord: LnCol) -> Result<Self::Char, Self::Error> {
        let coord = coord.advance_line(self.start.pred_count());
        self.grid.char_at(coord)
    }
}

impl<BaseGrid> LineAt<LnNum> for CharGridSliceFrom<BaseGrid, LnNum>
where
    BaseGrid: LineAt<LnNum>,
{
    type Line = BaseGrid::Line;
    type Error = BaseGrid::Error;

    fn line_at(self, ln_num: LnNum) -> Result<Self::Line, Self::Error> {
        let ln_num = self.start.advance_by(ln_num.pred_count());
        self.grid.line_at(ln_num)
    }
}

impl<BaseGrid> SliceFrom<LnNum> for CharGridSliceFrom<BaseGrid, LnNum>
where
    BaseGrid: SliceFrom<LnNum>,
{
    type Slice = BaseGrid::Slice;
    type Error = BaseGrid::Error;

    fn slice_from(self, start: LnNum) -> Result<Self::Slice, Self::Error> {
        let start = self.start.advance_by(start.pred_count());
        self.grid.slice_from(start)
    }
}

impl<BaseGrid> SliceFrom<LnCol> for CharGridSliceFrom<BaseGrid, LnNum>
where
    BaseGrid: SliceFrom<LnCol>,
{
    type Slice = BaseGrid::Slice;
    type Error = BaseGrid::Error;

    fn slice_from(self, start: LnCol) -> Result<Self::Slice, Self::Error> {
        let start = start.advance_column(self.start.pred_count());
        self.grid.slice_from(start)
    }
}

impl<BaseGrid> LineCount for CharGridSliceFrom<BaseGrid, LnNum>
where
    BaseGrid: CharCount,
{
    fn line_count(&self) -> usize {
        let total = self.grid.char_count();
        let skipped = self.start.pred_count();
        total - skipped
    }
}

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
