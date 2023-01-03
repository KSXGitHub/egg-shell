use crate::{CharAt, CharCoord, CharCount, ColNum, LineAt, LineCount, LnNum, SliceFrom};
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

impl<BaseGrid> CharAt<CharCoord> for CharGridSliceFrom<BaseGrid, LnNum>
where
    BaseGrid: CharAt<CharCoord>,
{
    type Char = BaseGrid::Char;
    type Error = BaseGrid::Error;

    fn char_at(self, coord: CharCoord) -> Result<Self::Char, Self::Error> {
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

impl<BaseGrid> SliceFrom<CharCoord> for CharGridSliceFrom<BaseGrid, LnNum>
where
    BaseGrid: SliceFrom<CharCoord>,
{
    type Slice = BaseGrid::Slice;
    type Error = BaseGrid::Error;

    fn slice_from(self, start: CharCoord) -> Result<Self::Slice, Self::Error> {
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

impl<BaseGrid> CharAt<CharCoord> for CharGridSliceFrom<BaseGrid, CharCoord>
where
    BaseGrid: CharAt<CharCoord>,
{
    type Char = BaseGrid::Char;
    type Error = BaseGrid::Error;

    fn char_at(self, coord: CharCoord) -> Result<Self::Char, Self::Error> {
        let CharGridSliceFrom { grid, start } = self;
        let coord = match coord.line.pred_count() {
            0 => start.advance_column(coord.column.pred_count()),
            _ => coord.advance_line(start.line.pred_count()),
        };
        grid.char_at(coord)
    }
}

impl<BaseGrid> LineAt<LnNum> for CharGridSliceFrom<BaseGrid, CharCoord>
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

impl<BaseGrid> SliceFrom<CharCoord> for CharGridSliceFrom<BaseGrid, CharCoord> {
    type Slice = CharGridSliceFrom<Self, CharCoord>;
    type Error = Infallible;
    fn slice_from(self, start: CharCoord) -> Result<Self::Slice, Self::Error> {
        Ok(CharGridSliceFrom { grid: self, start })
    }
}

impl<BaseGrid> LineCount for CharGridSliceFrom<BaseGrid, CharCoord>
where
    BaseGrid: LineCount,
{
    fn line_count(&self) -> usize {
        let total = self.grid.line_count();
        let skipped = self.start.column.pred_count();
        total - skipped
    }
}
