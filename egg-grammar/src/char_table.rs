use crate::TextSegment;
use assert_cmp::debug_assert_op;
use getset::{CopyGetters, Getters};
use pipe_trait::Pipe;
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
    #[strum(serialize = "")]
    EOF,
}

/// Loading progress of [`CharTable`].
#[derive(Clone)]
struct CharTableLoadingProgress<CharIter> {
    /// Source of characters to scan.
    src_char_iter: CharIter,
    /// Track the previously loaded character that isn't "\n".
    /// * `Some(char)` means that `char` is the previous character and `char` isn't "\n".
    /// * `None` means that the previous character is "\n".
    prev_non_lf: Option<char>,
    /// Byte offset of the previously loaded line.
    prev_line_offset: usize,
}

/// State of [`CharTable`].
///
/// `Some` means that the table is incomplete.
///
/// `None` means that the table is completed.
type CharTableCompletion<CharIter> = Option<CharTableLoadingProgress<CharIter>>;

/// Whether the [`CharTable`] is completed.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, AsRefStr, IntoStaticStr)]
pub enum CompletionStatus {
    /// Not all characters are loaded.
    Incomplete,
    /// All characters are loaded.
    Complete,
}

/// Table of characters.
#[derive(Clone, CopyGetters, Getters)]
pub struct CharTable<CharIter> {
    /// Total number of loaded characters.
    #[getset(get_copy = "pub")]
    loaded_char_count: usize,
    /// Loaded text so far.
    #[getset(get = "pub")]
    loaded_text: String,
    /// List of loaded lines.
    #[getset(get = "pub")]
    loaded_line_list: Vec<(TextSegment, EndOfLine)>,
    /// State of the table.
    ///
    /// `Some` means that the table is incomplete.
    ///
    /// `None` means that the table is completed.
    completion_progress: CharTableCompletion<CharIter>,
}

impl<CharIter> CharTable<CharIter> {
    /// Start loading characters into a new character table.
    pub const fn from_char_iter(src_char_iter: CharIter) -> Self {
        let state = Some(CharTableLoadingProgress {
            src_char_iter,
            prev_non_lf: None,
            prev_line_offset: 0,
        });
        CharTable {
            loaded_char_count: 0,
            loaded_text: String::new(),
            loaded_line_list: Vec::new(),
            completion_progress: state,
        }
    }

    /// Start loading characters into a new character table.
    pub fn from_char_list<CharList>(src_char_list: CharList) -> Self
    where
        CharList: IntoIterator<IntoIter = CharIter>,
    {
        CharTable::from_char_iter(src_char_list.into_iter())
    }

    /// Number of lines.
    pub fn loaded_line_count(&self) -> usize {
        self.loaded_line_list().len()
    }

    /// Whether the table is completed.
    pub const fn completion(&self) -> CompletionStatus {
        match self.completion_progress {
            Some(_) => CompletionStatus::Incomplete,
            None => CompletionStatus::Complete,
        }
    }
}

/// Success value of [`CharTable::load_char`].
#[derive(Debug, Clone, Copy)]
pub enum LoadCharReport<'a> {
    /// The table is completed.
    Document,
    /// Complete a line.
    Line(&'a str, EndOfLine),
    /// Get another character.
    Char(char),
}

/// Failure value of [`CharTable::load_char`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadCharError {
    /// Encounter an invalid character.
    IllPlacedCarriageReturn { followed_by: char },
}

impl<CharIter: Iterator<Item = char>> CharTable<CharIter> {
    /// Add another character to the table.
    pub fn load_char(&mut self) -> Result<LoadCharReport<'_>, LoadCharError> {
        let CharTable {
            loaded_char_count,
            loaded_text,
            loaded_line_list,
            completion_progress,
        } = self;

        let Some(CharTableLoadingProgress {
            src_char_iter,
            prev_non_lf,
            prev_line_offset,
        }) = completion_progress else {
            return Ok(LoadCharReport::Document);
        };

        let Some(char) = src_char_iter.next() else {
            let line_offset = *prev_line_offset;
            let line_src_text = &loaded_text[line_offset..];
            let line_segment = TextSegment::scan_text(line_src_text, loaded_line_list.len(), line_offset);
            loaded_line_list.push((line_segment, EndOfLine::EOF));
            *completion_progress = None;
            return Ok(LoadCharReport::Document);
        };

        let current_byte_offset = loaded_text.len();
        loaded_text.push(char);

        if char == '\n' {
            // TODO: refactor
            let last_char = *prev_non_lf;
            let (eol_offset, eol) = if last_char == Some('\r') {
                debug_assert_op!(current_byte_offset > 0);
                (current_byte_offset - 1, EndOfLine::CRLF)
            } else {
                (current_byte_offset, EndOfLine::LF)
            };
            let line_offset = *prev_line_offset;
            let line_src_text = &loaded_text[line_offset..eol_offset];
            let line_segment =
                TextSegment::scan_text(line_src_text, loaded_line_list.len(), line_offset);
            loaded_line_list.push((line_segment, eol));
            *loaded_char_count += 1;
            *prev_non_lf = None;
            *prev_line_offset = loaded_text.len();
            LoadCharReport::Line(line_src_text, eol).pipe(Ok)
        } else {
            // TODO: refactor
            if *prev_non_lf == Some('\r') {
                dbg!(loaded_text);
                return Err(LoadCharError::IllPlacedCarriageReturn { followed_by: char });
            }
            *loaded_char_count += 1;
            *prev_non_lf = Some(char);
            char.pipe(LoadCharReport::Char).pipe(Ok)
        }
    }

    /// Load the whole text.
    pub fn load_all(&mut self) -> Result<(), LoadCharError> {
        while self.completion() == CompletionStatus::Incomplete {
            self.load_char()?;
        }
        Ok(())
    }

    /// Return a table with completed text.
    pub fn into_completed(mut self) -> Result<Self, LoadCharError> {
        self.load_all()?;
        Ok(self)
    }
}

impl<CharIter> Debug for CharTable<CharIter> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let char_count = self.loaded_char_count();
        let line_count = self.loaded_line_count();
        let completion = self.completion();
        write!(
            f,
            "CharTable of {line_count} lines {char_count} chars ({completion})",
        )
    }
}

impl CharTable<std::str::Chars<'static>> {
    /// Start load characters from a static string.
    pub fn from_static_str(src_text: &'static str) -> Self {
        src_text.chars().pipe(CharTable::from_char_iter)
    }
}
