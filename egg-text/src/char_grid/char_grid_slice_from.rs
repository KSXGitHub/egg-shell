mod by_char_pos;
mod by_col_num;
mod by_ln_col;
mod by_ln_num;

pub use by_char_pos::*;
pub use by_col_num::*;
pub use by_ln_col::*;
pub use by_ln_num::*;

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
