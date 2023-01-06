use crate::{CharCell, CharOrEol, EndOfLine, TextSliceDef};
use derive_more::IsVariant;
use getset::{CopyGetters, Getters};
use std::fmt::Debug;
use strum::{AsRefStr, Display, IntoStaticStr};

/// Loading progress of [`LazyCharGrid`].
#[derive(Clone)]
pub(super) struct LoadingProgress<CharIter> {
    /// Source of characters to scan.
    pub(super) src_char_iter: CharIter,
    /// Track the previously loaded character that isn't "\n".
    /// * `Some(char)` means that `char` is the previous character and `char` isn't "\n".
    /// * `None` means that the previous character is "\n".
    pub(super) prev_non_lf: Option<char>,
    /// Byte offset of the previously loaded line.
    pub(super) prev_line_offset: usize,
}

/// State of [`LazyCharGrid`].
///
/// `Some` means that the grid is incomplete.
///
/// `None` means that the grid is completed.
type CompletionProgress<CharIter> = Option<LoadingProgress<CharIter>>;

/// Whether the [`LazyCharGrid`](super::LazyCharGrid) is completed.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, AsRefStr, IntoStaticStr, IsVariant)]
pub enum CompletionStatus {
    /// Not all characters are loaded.
    Incomplete,
    /// All characters are loaded.
    Complete,
}

/// Inner data of [`LazyCharGrid`](super::LazyCharGrid).
#[derive(CopyGetters, Getters)]
pub struct LazyCharGridData<CharIter> {
    /// Loaded text so far.
    #[getset(get = "pub")]
    pub(super) loaded_text: String,
    /// List of loaded character cells.
    #[getset(get = "pub")]
    pub(super) loaded_char_list: Vec<CharCell<CharOrEol>>, // TODO: reduce memory cost by storing only big characters.
    /// List of loaded line coordinates.
    #[getset(get = "pub")]
    pub(super) loaded_line_list: Vec<(TextSliceDef, EndOfLine)>,
    /// State of the grid.
    ///
    /// `Some` means that the grid is incomplete.
    ///
    /// `None` means that the grid is completed.
    pub(super) completion_progress: CompletionProgress<CharIter>,
}
