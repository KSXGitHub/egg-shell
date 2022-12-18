use crate::TextSegment;
use getset::{CopyGetters, Getters};
use std::fmt::{self, Debug, Formatter};
use strum::{AsRefStr, Display, IntoStaticStr};

/// String that ends a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr, Display, IntoStaticStr)]
#[allow(clippy::upper_case_acronyms)]
pub enum EndOfLine {
    #[strum(serialize = "\n")]
    LF,
    #[strum(serialize = "\r\n")]
    CRLF,
    #[strum(serialize = "\n\r")]
    LFCR,
    #[strum(serialize = "")]
    EOF,
}

/// Table of characters.
#[derive(Clone, CopyGetters, Getters)]
pub struct CharTable<CharIter> {
    /// Source of characters to scan.
    #[getset(get = "pub")]
    src_char_iter: CharIter,
    /// Byte offset of the last loaded line.
    #[getset(get_copy = "pub")]
    loaded_last_line_offset: usize,
    /// Total number of loaded characters.
    #[getset(get_copy = "pub")]
    loaded_char_count: usize,
    /// Loaded text so far.
    #[getset(get = "pub")]
    loaded_text: String,
    /// List of loaded lines.
    #[getset(get = "pub")]
    loaded_line_list: Vec<(TextSegment, EndOfLine)>,
    /// Whether the loading process is completed.
    #[getset(get_copy = "pub")]
    completed: bool,
}

impl<CharIter> CharTable<CharIter> {
    /// Start loading characters into a new character table.
    pub const fn from_char_iter(src_char_iter: CharIter) -> Self {
        CharTable {
            src_char_iter,
            loaded_last_line_offset: 0,
            loaded_char_count: 0,
            loaded_text: String::new(),
            loaded_line_list: Vec::new(),
            completed: false,
        }
    }

    /// Number of lines.
    pub fn loaded_line_count(&self) -> usize {
        self.loaded_line_list().len()
    }
}

/// Result of [`CharTable::load_char`].
#[derive(Debug, Clone, Copy)]
pub enum ScanNextCharResult<'a> {
    /// The table is completed.
    Document,
    /// Complete a line.
    Line(&'a str, EndOfLine),
    /// Get another character.
    Char(char),
}

impl<CharIter: Iterator<Item = char>> CharTable<CharIter> {
    /// Add another character to the table.
    pub fn load_char(&mut self) -> ScanNextCharResult<'_> {
        let CharTable {
            src_char_iter,
            loaded_last_line_offset,
            loaded_char_count,
            loaded_text,
            loaded_line_list,
            completed,
        } = self;

        let Some(char) = src_char_iter.next() else {
            *completed = true;
            return ScanNextCharResult::Document;
        };

        if char == '\n' {
            // TODO: refactor
            let current_byte_offset = loaded_text.len();
            let last_byte_offset = current_byte_offset - 1;
            let (eol_offset, eol) = if loaded_text.get(last_byte_offset..) == Some("\r") {
                (last_byte_offset, EndOfLine::CRLF)
            } else {
                (current_byte_offset, EndOfLine::LF)
            };
            let line_offset = *loaded_last_line_offset;
            let line_src_text = &loaded_text[line_offset..eol_offset];
            let line_segment =
                TextSegment::scan_text(line_src_text, loaded_line_list.len(), line_offset);
            loaded_line_list.push((line_segment, eol));
            *loaded_char_count += 1;
            ScanNextCharResult::Line(line_src_text, eol)
        } else if char == '\r' {
            // TODO: refactor
            let current_byte_offset = loaded_text.len();
            let last_byte_offset = current_byte_offset - 1;
            if loaded_text.get(last_byte_offset..) != Some("\n") {
                *loaded_char_count += 1;
                return ScanNextCharResult::Char(char);
            }
            let line_offset = *loaded_last_line_offset;
            let line_src_text = &loaded_text[line_offset..last_byte_offset];
            let line_segment =
                TextSegment::scan_text(line_src_text, loaded_line_list.len(), line_offset);
            loaded_line_list.push((line_segment, EndOfLine::LFCR));
            *loaded_char_count += 1;
            ScanNextCharResult::Line(line_src_text, EndOfLine::LFCR)
        } else {
            // TODO: refactor
            *loaded_char_count += 1;
            ScanNextCharResult::Char(char)
        }
    }

    /// Load the whole text.
    pub fn load_all(&mut self) {
        while !self.completed() {
            self.load_char();
        }
    }
}

impl<CharIter> Debug for CharTable<CharIter> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let char_count = self.loaded_char_count();
        let line_count = self.loaded_line_count();
        let completion = if self.completed() {
            "complete"
        } else {
            "incomplete"
        };
        write!(
            f,
            "CharTable of {line_count} lines {char_count} chars ({completion})",
        )
    }
}
