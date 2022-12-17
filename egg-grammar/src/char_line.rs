use crate::{CharCell, CharCoord, Ordinal};
use derive_more::Display;
use getset::CopyGetters;
use std::fmt::{self, Debug, Formatter};

/// Information of a single line.
#[derive(Display, Clone, CopyGetters)]
#[display(fmt = "{src_text}")]
#[getset(get_copy = "pub")]
pub struct CharLine<'a> {
    /// Position of the line.
    pos: Ordinal,
    /// Total sizes of all lines before this line.
    offset: usize,
    /// Source text of the line.
    src_text: &'a str,
    /// List of characters.
    #[getset(skip)]
    char_list: Vec<CharCell>,
}

impl<'a> CharLine<'a> {
    /// Scan a line of text.
    pub(crate) fn scan_text(src_text: &'a str, ln_pred: usize, offset: usize) -> Self {
        let pos = Ordinal::from_pred_count(ln_pred);
        let mut offset_from_ln_start = 0;
        let mut char_list = Vec::new();
        for (col_pred, value) in src_text.chars().enumerate() {
            char_list.push(CharCell {
                coord: CharCoord::from_pred_counts(ln_pred, col_pred),
                offset_from_doc_start: offset + offset_from_ln_start,
                offset_from_ln_start,
                value,
            });
            offset_from_ln_start += value.len_utf8();
        }
        CharLine {
            pos,
            offset,
            src_text,
            char_list,
        }
    }

    /// Number of characters.
    pub fn char_count(&self) -> usize {
        self.char_list.len()
    }

    /// Iterate over all character cells in the line.
    pub fn char_cells(&self) -> impl Iterator<Item = &CharCell> {
        self.char_list.iter()
    }
}

impl<'a> Debug for CharLine<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let CharLine { pos, src_text, .. } = self;
        write!(f, "CharLine at {pos} of {src_text:?}")
    }
}
