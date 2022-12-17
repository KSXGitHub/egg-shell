use crate::{CharCoord, Ordinal};
use getset::CopyGetters;

/// Information of a single character.
#[derive(Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct CharCell {
    /// Content of the character.
    value: char,
    /// Character coordinate.
    coord: CharCoord,
}

/// Information of a single line.
#[derive(Clone, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct CharLine<'a> {
    /// Position of the line.
    pos: Ordinal,
    /// Source text of the line.
    src_text: &'a str,
    /// List of characters.
    #[getset(skip)]
    char_list: Vec<CharCell>,
}

impl<'a> CharLine<'a> {
    /// Scan a line of text.
    fn scan_text(src_text: &'a str, ln_offset: usize) -> Self {
        let pos = Ordinal::from_offset(ln_offset);
        let char_list = src_text
            .chars()
            .enumerate()
            .map(|(col_offset, value)| (CharCoord::from_offsets(ln_offset, col_offset), value))
            .map(|(coord, value)| CharCell { value, coord })
            .collect();
        CharLine {
            pos,
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

/// Table of characters.
#[derive(Clone, CopyGetters)]
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
        let line_iter = src_text
            .lines()
            .enumerate()
            .map(|(ln_offset, ln_text)| CharLine::scan_text(ln_text, ln_offset));
        let mut char_count = 0;
        let mut line_list = Vec::new();
        for char_line in line_iter {
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
}
