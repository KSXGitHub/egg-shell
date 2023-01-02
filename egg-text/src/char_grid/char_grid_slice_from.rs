use crate::{CharAt, CharCoord, CharCount, ColumnNumber, LineAt, LineCount, LineNumber, SliceFrom};

// TODO: this implementation is absolutely wrong, fix this

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

impl<BaseGrid> CharAt<ColumnNumber> for CharGridSliceFrom<BaseGrid, ColumnNumber>
where
    BaseGrid: CharAt<ColumnNumber>,
{
    type Char = BaseGrid::Char;
    type Error = BaseGrid::Error;

    fn char_at(self, col_num: ColumnNumber) -> Result<Self::Char, Self::Error> {
        let col_num = self.start.advance_by(col_num.pred_count());
        self.grid.char_at(col_num)
    }
}

impl<BaseGrid> SliceFrom<ColumnNumber> for CharGridSliceFrom<BaseGrid, ColumnNumber>
where
    BaseGrid: SliceFrom<ColumnNumber>,
{
    type Slice = BaseGrid::Slice;
    type Error = BaseGrid::Error;

    fn slice_from(self, start: ColumnNumber) -> Result<Self::Slice, Self::Error> {
        let start = self.start.advance_by(start.pred_count());
        self.grid.slice_from(start)
    }
}

impl<BaseGrid> CharCount for CharGridSliceFrom<BaseGrid, ColumnNumber>
where
    BaseGrid: CharCount,
{
    fn char_count(&self) -> usize {
        let total = self.grid.char_count();
        let skipped = self.start.pred_count();
        total - skipped
    }
}

impl<BaseGrid> CharAt<CharCoord> for CharGridSliceFrom<BaseGrid, LineNumber>
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

impl<BaseGrid> LineAt<LineNumber> for CharGridSliceFrom<BaseGrid, LineNumber>
where
    BaseGrid: LineAt<LineNumber>,
{
    type Line = BaseGrid::Line;
    type Error = BaseGrid::Error;

    fn line_at(self, ln_num: LineNumber) -> Result<Self::Line, Self::Error> {
        let ln_num = self.start.advance_by(ln_num.pred_count());
        self.grid.line_at(ln_num)
    }
}

impl<BaseGrid> SliceFrom<LineNumber> for CharGridSliceFrom<BaseGrid, LineNumber>
where
    BaseGrid: SliceFrom<LineNumber>,
{
    type Slice = BaseGrid::Slice;
    type Error = BaseGrid::Error;

    fn slice_from(self, start: LineNumber) -> Result<Self::Slice, Self::Error> {
        let start = self.start.advance_by(start.pred_count());
        self.grid.slice_from(start)
    }
}

impl<BaseGrid> SliceFrom<CharCoord> for CharGridSliceFrom<BaseGrid, LineNumber>
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

impl<BaseGrid> LineCount for CharGridSliceFrom<BaseGrid, LineNumber>
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
        let coord = match coord.line.pred_count() {
            0 => self.start.advance_column(coord.column.pred_count()),
            _ => self.start.advance_line(coord.line.pred_count()),
        };
        self.grid.char_at(coord)
    }
}

impl<BaseGrid> LineAt<LineNumber> for CharGridSliceFrom<BaseGrid, CharCoord>
where
    BaseGrid: LineAt<LineNumber>,
{
    type Line = BaseGrid::Line;
    type Error = BaseGrid::Error;

    fn line_at(self, ln_num: LineNumber) -> Result<Self::Line, Self::Error> {
        let ln_num = self.start.line.advance_by(ln_num.pred_count());
        self.grid.line_at(ln_num)
    }
}

impl<BaseGrid> SliceFrom<CharCoord> for CharGridSliceFrom<BaseGrid, CharCoord>
where
    BaseGrid: SliceFrom<CharCoord>,
{
    type Slice = BaseGrid::Slice;
    type Error = BaseGrid::Error;

    fn slice_from(self, start: CharCoord) -> Result<Self::Slice, Self::Error> {
        let start = self
            .start
            .advance_line(start.line.pred_count())
            .advance_column(start.column.pred_count());
        self.grid.slice_from(start)
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
