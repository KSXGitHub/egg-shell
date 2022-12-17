use crate::{CharCoord, Ordinal};
use derive_more::Display;
use getset::CopyGetters;
use std::fmt::{self, Debug, Formatter};

/// Information of a single character.
#[derive(Display, Clone, Copy, CopyGetters)]
#[display(fmt = "{value}")]
#[getset(get_copy = "pub")]
pub struct CharCell {
    /// Content of the character.
    value: char,
    /// Character coordinate.
    coord: CharCoord,
}

impl Debug for CharCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let CharCell { value, coord } = self;
        write!(f, "CharCell at {coord} of {value:?}")
    }
}

/// Information of a single line.
#[derive(Display, Clone, CopyGetters)]
#[display(fmt = "{src_text}")]
#[getset(get_copy = "pub")]
pub struct CharLine<'a> {
    /// Position of the line.
    pos: Ordinal,
    /// Total sizes of lines before this line.
    offset: usize,
    /// Source text of the line.
    src_text: &'a str,
    /// List of characters.
    #[getset(skip)]
    char_list: Vec<CharCell>,
}

impl<'a> CharLine<'a> {
    /// Scan a line of text.
    fn scan_text(src_text: &'a str, ln_pred: usize, offset: usize) -> Self {
        let pos = Ordinal::from_pred_count(ln_pred);
        let char_list = src_text
            .chars()
            .enumerate()
            .map(|(col_pred, value)| (CharCoord::from_pred_counts(ln_pred, col_pred), value))
            .map(|(coord, value)| CharCell { value, coord })
            .collect();
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
