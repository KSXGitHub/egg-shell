use crate::{CharCell, CharLine};
use derive_more::Display;
use getset::CopyGetters;
use std::fmt::{self, Debug, Formatter};

/// Table of characters.
#[derive(Display, Clone, CopyGetters)]
#[display(fmt = "{src_text}")]
#[getset(get_copy = "pub")]
pub struct CharTable<'a> {
    /// Total number of characters.
    char_count: usize,
    /// Source text.
    src_text: &'a str,
    /// List of lines.
    #[getset(skip)]
    line_list: Vec<CharLine<'a>>,
}

impl<'a> CharTable<'a> {
    /// Create character table from scanning a document.
    pub fn scan_text(src_text: &'a str) -> Self {
        let mut offset = 0;
        let mut char_count = 0;
        let mut line_list = Vec::new();
        for (ln_pred, ln_text) in src_text.lines().enumerate() {
            let char_line = CharLine::scan_text(ln_text, ln_pred, offset);
            offset += ln_text.len();
            char_count += char_line.char_count();
            line_list.push(char_line);
        }
        CharTable {
            char_count,
            src_text,
            line_list,
        }
    }

    /// Iterate over all lines in the table.
    pub fn char_lines(&self) -> impl Iterator<Item = &CharLine<'a>> {
        self.line_list.iter()
    }

    /// Iterate over all character cells in the table.
    pub fn char_cells(&self) -> impl Iterator<Item = &CharCell> {
        self.char_lines().flat_map(CharLine::char_cells)
    }

    /// Number of lines.
    pub fn line_count(&self) -> usize {
        self.line_list.len()
    }
}

impl<'a> Debug for CharTable<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let char_count = self.char_count();
        let line_count = self.line_count();
        write!(f, "CharTable of {line_count} lines {char_count} chars")
    }
}
