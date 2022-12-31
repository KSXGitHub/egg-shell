use crate::{CharAt, CharCoord, LineAt, LineCount, Ordinal, SliceFrom};
use std::ops::Deref;

/// Create a slice of char grid from a start coordinate.
///
/// The resulting slice includes the start coordinate.
#[derive(Debug, Clone, Copy)]
pub struct CharGridSliceFrom<BaseGridRef> {
    /// Reference grid.
    pub grid: BaseGridRef,
    /// Start coordinate.
    pub start: CharCoord,
}

impl<'a, BaseGridRef> CharAt<'a> for CharGridSliceFrom<BaseGridRef>
where
    BaseGridRef: Deref + 'a,
    BaseGridRef::Target: CharAt<'a>,
{
    type Char = <BaseGridRef::Target as CharAt<'a>>::Char;
    type Error = <BaseGridRef::Target as CharAt<'a>>::Error;

    fn char_at(&'a self, coord: CharCoord) -> Result<Self::Char, Self::Error> {
        let coord = self
            .start
            .advance_line(coord.line.pred_count())
            .advance_column(coord.column.pred_count());
        self.grid.char_at(coord)
    }
}

impl<'a, BaseGridRef> LineAt<'a> for CharGridSliceFrom<BaseGridRef>
where
    BaseGridRef: Deref + 'a,
    BaseGridRef::Target: LineAt<'a>,
{
    type Line = <BaseGridRef::Target as LineAt<'a>>::Line;
    type Error = <BaseGridRef::Target as LineAt<'a>>::Error;

    fn line_at(&'a self, ln_num: Ordinal) -> Result<Self::Line, Self::Error> {
        let ln_num = self.start.line.advance_by(ln_num.pred_count());
        self.grid.line_at(ln_num)
    }
}

impl<'a, BaseGridRef> SliceFrom<'a> for CharGridSliceFrom<BaseGridRef>
where
    BaseGridRef: Deref + 'a,
    BaseGridRef::Target: SliceFrom<'a>,
{
    type Slice = <BaseGridRef::Target as SliceFrom<'a>>::Slice;
    type Error = <BaseGridRef::Target as SliceFrom<'a>>::Error;

    fn slice_from(&'a self, start: CharCoord) -> Result<Self::Slice, Self::Error> {
        let start = self
            .start
            .advance_line(start.line.pred_count())
            .advance_column(start.column.pred_count());
        self.grid.slice_from(start)
    }
}

impl<BaseGridRef> LineCount for CharGridSliceFrom<BaseGridRef>
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
