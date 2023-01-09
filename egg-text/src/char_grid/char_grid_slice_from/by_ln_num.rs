use super::CharGridSliceFrom;
use crate::{
    CharAt, CharCount, LineAt, LineCount, LnCol, LnNum, LnNumOutOfBound, SliceFrom, TryIterChar,
    TryIterLine,
};
use derive_more::{Display, Error};
use pipe_trait::Pipe;

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

/// Error type of [`TryIterChar`] on [`CharGridSliceFrom<_, LnNum>`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum LnNumIterCharError<IterLineError, IterColumnError> {
    AtLine(IterLineError),
    AtColumn(IterColumnError),
}

/// Character iterator of [`CharGridSliceFrom<_, LnNum>`].
pub struct LnNumCharIter<GridRef>
where
    GridRef: LineAt<LnNum> + Copy,
    GridRef::Error: TryInto<LnNumOutOfBound>,
    GridRef::Line: TryIterChar + Copy,
{
    ln_iter: <CharGridSliceFrom<GridRef, LnNum> as TryIterLine>::LineResultIter,
    col_iter: Option<<GridRef::Line as TryIterChar>::CharResultIter>,
}

impl<GridRef> Iterator for LnNumCharIter<GridRef>
where
    GridRef: LineAt<LnNum> + Copy,
    GridRef::Error: TryInto<LnNumOutOfBound>,
    GridRef::Line: TryIterChar + Copy,
{
    type Item = Result<
        <GridRef::Line as TryIterChar>::Char,
        LnNumIterCharError<
            <GridRef::Error as TryInto<LnNumOutOfBound>>::Error,
            <GridRef::Line as TryIterChar>::Error,
        >,
    >;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(col_iter) = &mut self.col_iter {
            let Some(col_item) = col_iter.next() else {
                self.col_iter = None;
                return self.next();
            };

            col_item.map_err(LnNumIterCharError::AtColumn).pipe(Some)
        } else {
            match self.ln_iter.next()? {
                Err(error) => error.pipe(LnNumIterCharError::AtLine).pipe(Err).pipe(Some),
                Ok(ln_item) => {
                    self.col_iter = Some(ln_item.try_iter_char());
                    self.next()
                }
            }
        }
    }
}

impl<BaseGridRef> TryIterChar for CharGridSliceFrom<BaseGridRef, LnNum>
where
    BaseGridRef: LineAt<LnNum> + Copy,
    BaseGridRef::Error: TryInto<LnNumOutOfBound>,
    BaseGridRef::Line: TryIterChar + Copy,
{
    type Char = <BaseGridRef::Line as TryIterChar>::Char;
    type Error = LnNumIterCharError<
        <BaseGridRef::Error as TryInto<LnNumOutOfBound>>::Error,
        <BaseGridRef::Line as TryIterChar>::Error,
    >;
    type CharResultIter = LnNumCharIter<BaseGridRef>;

    fn try_iter_char(self) -> Self::CharResultIter {
        LnNumCharIter {
            ln_iter: self.try_iter_line(),
            col_iter: None,
        }
    }
}

/// Line iterator of [`CharGridSliceFrom<_, LnNum>`].
pub struct LnNumLineIter<GridRef> {
    ln_num: LnNum,
    grid: GridRef,
}

impl<GridRef> Iterator for LnNumLineIter<GridRef>
where
    GridRef: LineAt<LnNum> + Copy,
    GridRef::Error: TryInto<LnNumOutOfBound>,
{
    type Item = Result<GridRef::Line, <GridRef::Error as TryInto<LnNumOutOfBound>>::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.grid.line_at(self.ln_num).map_err(TryInto::try_into) {
            Ok(line) => Ok(line),
            Err(Ok(LnNumOutOfBound)) => return None,
            Err(Err(error)) => Err(error),
        };
        self.ln_num = self.ln_num.advance_by(1);
        Some(item)
    }
}

impl<BaseGridRef> TryIterLine for CharGridSliceFrom<BaseGridRef, LnNum>
where
    BaseGridRef: LineAt<LnNum> + Copy,
    BaseGridRef::Error: TryInto<LnNumOutOfBound>,
{
    type Line = BaseGridRef::Line;
    type Error = <BaseGridRef::Error as TryInto<LnNumOutOfBound>>::Error;
    type LineResultIter = LnNumLineIter<BaseGridRef>;

    fn try_iter_line(self) -> Self::LineResultIter {
        LnNumLineIter {
            ln_num: self.start,
            grid: self.grid,
        }
    }
}
