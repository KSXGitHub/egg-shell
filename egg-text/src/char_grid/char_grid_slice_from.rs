use crate::{CharAt, CharCoord, LineAt, LineCount, LineNumber, SliceFrom};
use std::ops::Deref;

// TODO: this implementation is absolutely wrong, fix this

/// Create a slice of char grid from a start coordinate.
///
/// The resulting slice includes the start coordinate.
#[derive(Debug, Clone, Copy)]
pub struct CharGridSliceFrom<BaseGridRef, Coord> {
    /// Reference grid.
    pub grid: BaseGridRef,
    /// Start coordinate.
    pub start: Coord,
}

impl<'a, BaseGridRef> CharAt<CharCoord> for &'a CharGridSliceFrom<BaseGridRef, CharCoord>
where
    BaseGridRef: CharAt<CharCoord> + Copy + 'a,
{
    type Char = BaseGridRef::Char;
    type Error = BaseGridRef::Error;

    fn char_at(self, coord: CharCoord) -> Result<Self::Char, Self::Error> {
        let coord = self
            .start
            .advance_line(coord.line.pred_count())
            .advance_column(coord.column.pred_count());
        self.grid.char_at(coord)
    }
}

impl<'a, BaseGridRef> LineAt<LineNumber> for &'a CharGridSliceFrom<BaseGridRef, CharCoord>
where
    BaseGridRef: LineAt<LineNumber> + Copy + 'a,
{
    type Line = BaseGridRef::Line;
    type Error = BaseGridRef::Error;

    fn line_at(self, ln_num: LineNumber) -> Result<Self::Line, Self::Error> {
        let ln_num = self.start.line.advance_by(ln_num.pred_count());
        self.grid.line_at(ln_num)
    }
}

impl<'a, BaseGridRef> SliceFrom<'a, CharCoord> for CharGridSliceFrom<BaseGridRef, CharCoord>
where
    BaseGridRef: Deref + 'a,
    BaseGridRef::Target: SliceFrom<'a, CharCoord>,
{
    type Slice = <BaseGridRef::Target as SliceFrom<'a, CharCoord>>::Slice;
    type Error = <BaseGridRef::Target as SliceFrom<'a, CharCoord>>::Error;

    fn slice_from(&'a self, start: CharCoord) -> Result<Self::Slice, Self::Error> {
        let start = self
            .start
            .advance_line(start.line.pred_count())
            .advance_column(start.column.pred_count());
        self.grid.slice_from(start)
    }
}

impl<BaseGridRef> LineCount for CharGridSliceFrom<BaseGridRef, CharCoord>
where
    BaseGridRef: Deref,
    BaseGridRef::Target: LineCount,
{
    fn line_count(&self) -> usize {
        let total = self.grid.line_count();
        let skipped = self.start.column.pred_count();
        total - skipped
    }
}
