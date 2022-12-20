use crate::{text_slice::ScanText, CharCell, CharCoord, EndOfLine, TextSliceDef};
use assert_cmp::debug_assert_op;
use getset::{CopyGetters, Getters};
use pipe_trait::Pipe;
use std::{
    convert::Infallible,
    fmt::{self, Debug, Display, Formatter},
};
use strum::{AsRefStr, Display, IntoStaticStr};

/// Represent a line in the [`CharTable`].
#[derive(Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct CharTableLine<'a, CharIter> {
    /// Coordinate of the line
    coord: TextSliceDef,
    /// Type of EOL string.
    eol: EndOfLine,
    /// Reference table.
    table: &'a CharTable<CharIter>,
}

impl<'a, CharIter> CharTableLine<'a, CharIter> {
    /// Create a [`CharTableLine`].
    const fn new(coord: TextSliceDef, eol: EndOfLine, table: &'a CharTable<CharIter>) -> Self {
        CharTableLine { coord, eol, table }
    }

    /// Get text content of the slice without EOL.
    pub fn text_without_eol(&self) -> &'a str {
        let start = self.coord.offset();
        let end = start + self.coord.size();
        &self.table.loaded_text[start..end]
    }
}

impl<'a, CharIter> Display for CharTableLine<'a, CharIter> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let content = self.text_without_eol();
        let eol = self.eol;
        write!(f, "{content}{eol}")
    }
}

impl<'a, CharIter> Debug for CharTableLine<'a, CharIter> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let content = self.text_without_eol();
        let eol = self.eol;
        write!(f, "CharTableLine {content:?} {eol:?}")
    }
}

/// Loading progress of [`CharTable`].
#[derive(Clone)]
struct LoadingProgress<CharIter> {
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
type CompletionProgress<CharIter> = Option<LoadingProgress<CharIter>>;

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
    /// List of loaded character cells.
    #[getset(get = "pub")]
    loaded_char_list: Vec<CharCell>,
    /// List of loaded line coordinates.
    loaded_line_list: Vec<(TextSliceDef, EndOfLine)>,
    /// State of the table.
    ///
    /// `Some` means that the table is incomplete.
    ///
    /// `None` means that the table is completed.
    completion_progress: CompletionProgress<CharIter>,
}

impl<CharIter> CharTable<CharIter> {
    /// Allocating a character table and assign a stream to load from.
    ///
    /// **Parameters:**
    /// * `src_char_iter` is an iterator that emits [results](Result) of UTF-8 characters.
    /// * `capacity` is the capacity of the final text (e.g. file size of the source code).
    ///
    /// **Example:** Load from file
    ///
    /// ```rust,no_run
    /// use egg_grammar::CharTable;
    /// use std::{
    ///     fs::{metadata, File},
    ///     io::BufReader,
    /// };
    /// use utf8_chars::BufReadCharsExt;
    ///
    /// let file = File::open("my-file.txt").unwrap();
    /// let size: u64 = metadata("my-file.txt").unwrap().len();
    /// let size: usize = size.try_into().unwrap_or_else(|_| {
    ///     eprintln!("warning: {size}bytes is too big to allocate all at once,");
    ///     eprintln!("         the program can only handle part of the file");
    ///     0
    /// });
    /// let mut buf = BufReader::new(file);
    /// let char_iter = buf.chars_raw();
    /// let table = CharTable::new(char_iter, size);
    /// // ... do stuffs with table ...
    /// ```
    pub fn new(src_char_iter: CharIter, capacity: usize) -> Self {
        let state = Some(LoadingProgress {
            src_char_iter,
            prev_non_lf: None,
            prev_line_offset: 0,
        });
        CharTable {
            loaded_char_count: 0,
            loaded_text: String::with_capacity(capacity),
            loaded_char_list: Vec::with_capacity(capacity * std::mem::size_of::<char>()),
            loaded_line_list: Vec::new(),
            completion_progress: state,
        }
    }

    /// List all loaded lines.
    pub fn loaded_line_list(&self) -> impl Iterator<Item = CharTableLine<'_, CharIter>> {
        let create = |(coord, eol)| CharTableLine::new(coord, eol, self);
        self.loaded_line_list.iter().copied().map(create)
    }

    /// Number of lines.
    pub fn loaded_line_count(&self) -> usize {
        self.loaded_line_list.len()
    }

    /// Whether the table is completed.
    pub const fn completion(&self) -> CompletionStatus {
        match self.completion_progress {
            Some(_) => CompletionStatus::Incomplete,
            None => CompletionStatus::Complete,
        }
    }

    /// Return the total number of characters if the table is fully loaded.
    /// * `Some(n)` means that the table is fully loaded with `n` characters.
    /// * `None` means that the table isn't yet completed.
    pub const fn total_char_count(&self) -> Option<usize> {
        match self.completion() {
            CompletionStatus::Complete => Some(self.loaded_char_count),
            CompletionStatus::Incomplete => None,
        }
    }

    /// Return reference to the full string if the table is fully loaded.
    /// * `Some(text)` means that the table is fully loaded with `text` being the content.
    /// * `None` means that the table isn't yet completed.
    pub const fn full_text(&self) -> Option<&String> {
        match self.completion() {
            CompletionStatus::Complete => Some(&self.loaded_text),
            CompletionStatus::Incomplete => None,
        }
    }

    /// Return reference to the complete list of lines if the table is fully loaded.
    /// * `Some(list)` means that the table is fully loaded with `list` being the complete list of lines.
    /// * `None` means that the table isn't yet completed.
    pub fn all_lines(&self) -> Option<impl Iterator<Item = CharTableLine<'_, CharIter>>> {
        match self.completion() {
            CompletionStatus::Complete => Some(self.loaded_line_list()),
            CompletionStatus::Incomplete => None,
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
pub enum LoadCharError<IterError> {
    /// Encounter an invalid character.
    IllPlacedCarriageReturn { followed_by: char },
    /// Error emitted by character iterator.
    IterError(IterError),
}

impl<IterError, CharIter: Iterator<Item = Result<char, IterError>>> CharTable<CharIter> {
    /// Add another character to the table.
    pub fn load_char(&mut self) -> Result<LoadCharReport<'_>, LoadCharError<IterError>> {
        let CharTable {
            loaded_char_count,
            loaded_text,
            loaded_char_list,
            loaded_line_list,
            completion_progress,
        } = self;

        let Some(LoadingProgress {
            src_char_iter,
            prev_non_lf,
            prev_line_offset,
        }) = completion_progress else {
            return Ok(LoadCharReport::Document);
        };

        let Some(char) = src_char_iter.next() else {
            let line_offset = *prev_line_offset;
            let line_src_text = &loaded_text[line_offset..];
            let line_slice_def = ScanText::run(ScanText {
                char_list: loaded_char_list,
                src_text: line_src_text,
                first_char_coord: CharCoord::from_pred_counts(loaded_line_list.len(), 0),
                offset: line_offset
            });
            loaded_line_list.push((line_slice_def, EndOfLine::EOF));
            loaded_line_list.shrink_to_fit(); // The list is final (no more changes), it is safe to shrink to free some memory
            *completion_progress = None;
            return Ok(LoadCharReport::Document);
        };

        let char = char.map_err(LoadCharError::IterError)?;

        let current_byte_offset = loaded_text.len();
        loaded_text.push(char);

        if char == '\n' {
            let last_char = *prev_non_lf;
            let (eol_offset, eol) = if last_char == Some('\r') {
                debug_assert_op!(current_byte_offset > 0);
                (current_byte_offset - 1, EndOfLine::CRLF)
            } else {
                (current_byte_offset, EndOfLine::LF)
            };
            let line_offset = *prev_line_offset;
            let line_src_text = &loaded_text[line_offset..eol_offset];
            let line_slice_def = ScanText::run(ScanText {
                char_list: loaded_char_list,
                src_text: line_src_text,
                first_char_coord: CharCoord::from_pred_counts(loaded_line_list.len(), 0),
                offset: line_offset,
            });
            loaded_line_list.push((line_slice_def, eol));
            *loaded_char_count += 1;
            *prev_non_lf = None;
            *prev_line_offset = loaded_text.len();
            LoadCharReport::Line(line_src_text, eol).pipe(Ok)
        } else {
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
    pub fn load_all(&mut self) -> Result<(), LoadCharError<IterError>> {
        while self.completion() == CompletionStatus::Incomplete {
            self.load_char()?;
        }
        Ok(())
    }

    /// Return a table with completed text.
    pub fn into_completed(mut self) -> Result<Self, LoadCharError<IterError>> {
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

impl CharTable<Infallible> {
    /// Allocating a character table and assign a stream to load from.
    ///
    /// Unlike [`new()`](CharTable::new), this constructor takes an infallible iterator.
    ///
    /// **Parameters:**
    /// * `src_char_iter` is an iterator that emits UTF-8 characters.
    /// * `capacity` is the capacity of the final text (e.g. file size of the source code).
    pub fn new_infallible<InfallibleCharIter>(
        src_char_iter: InfallibleCharIter,
        capacity: usize,
    ) -> CharTable<impl Iterator<Item = Result<char, Infallible>>>
    where
        InfallibleCharIter: IntoIterator<Item = char>,
    {
        let char_iter = src_char_iter.into_iter().map(Result::Ok);
        CharTable::new(char_iter, capacity)
    }

    /// Start load characters from a string slice.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(
        src_text: &str,
    ) -> CharTable<impl Iterator<Item = Result<char, Infallible>> + '_> {
        CharTable::new_infallible(src_text.chars(), src_text.len())
    }
}
