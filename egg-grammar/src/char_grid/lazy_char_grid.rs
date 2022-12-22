use super::{CharGridLine, CompletedCharGrid};
use crate::{text_slice::ScanText, CharCell, CharCoord, EndOfLine, TextSliceDef};
use assert_cmp::debug_assert_op;
use derive_more::Error;
use getset::{CopyGetters, Getters};
use pipe_trait::Pipe;
use std::{
    convert::Infallible,
    fmt::{self, Debug, Formatter},
};
use strum::{AsRefStr, Display, IntoStaticStr};

/// Loading progress of [`LazyCharGrid`].
#[derive(Clone)]
pub(super) struct LoadingProgress<CharIter> {
    /// Source of characters to scan.
    src_char_iter: CharIter,
    /// Track the previously loaded character that isn't "\n".
    /// * `Some(char)` means that `char` is the previous character and `char` isn't "\n".
    /// * `None` means that the previous character is "\n".
    prev_non_lf: Option<char>,
    /// Byte offset of the previously loaded line.
    prev_line_offset: usize,
}

/// State of [`LazyCharGrid`].
///
/// `Some` means that the grid is incomplete.
///
/// `None` means that the grid is completed.
type CompletionProgress<CharIter> = Option<LoadingProgress<CharIter>>;

/// Whether the [`LazyCharGrid`] is completed.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, AsRefStr, IntoStaticStr)]
pub enum CompletionStatus {
    /// Not all characters are loaded.
    Incomplete,
    /// All characters are loaded.
    Complete,
}

/// Grid of characters.
#[derive(Clone, CopyGetters, Getters)]
pub struct LazyCharGrid<CharIter> {
    /// Total number of loaded characters.
    #[getset(get_copy = "pub")]
    pub(super) loaded_char_count: usize,
    /// Loaded text so far.
    #[getset(get = "pub")]
    pub(super) loaded_text: String,
    /// List of loaded character cells.
    #[getset(get = "pub")]
    pub(super) loaded_char_list: Vec<CharCell>,
    /// List of loaded line coordinates.
    pub(super) loaded_line_list: Vec<(TextSliceDef, EndOfLine)>,
    /// State of the grid.
    ///
    /// `Some` means that the grid is incomplete.
    ///
    /// `None` means that the grid is completed.
    pub(super) completion_progress: CompletionProgress<CharIter>,
}

impl<CharIter> LazyCharGrid<CharIter> {
    /// Allocating a character grid and assign a stream to load from.
    ///
    /// **Parameters:**
    /// * `src_char_iter` is an iterator that emits [results](Result) of UTF-8 characters.
    /// * `capacity` is the capacity of the final text (e.g. file size of the source code).
    ///
    /// **Example:** Load from file
    ///
    /// ```rust,no_run
    /// use egg_grammar::LazyCharGrid;
    /// use std::{
    ///     fs::{metadata, File},
    ///     io::BufReader,
    /// };
    /// use utf8_chars::BufReadCharsExt;
    ///
    /// let file = File::open("my-file.txt").unwrap();
    /// let size: u64 = file.metadata().unwrap().len();
    /// let size: usize = size.try_into().unwrap_or_else(|_| {
    ///     eprintln!("warning: {size}bytes is too big to allocate all at once,");
    ///     eprintln!("         the program can only handle part of the file");
    ///     0
    /// });
    /// let mut buf = BufReader::new(file);
    /// let char_iter = buf.chars_raw();
    /// let grid = LazyCharGrid::new(char_iter, size);
    /// // ... do stuffs with grid ...
    /// ```
    pub fn new(src_char_iter: CharIter, capacity: usize) -> Self {
        let state = Some(LoadingProgress {
            src_char_iter,
            prev_non_lf: None,
            prev_line_offset: 0,
        });
        LazyCharGrid {
            loaded_char_count: 0,
            loaded_text: String::with_capacity(capacity),
            loaded_char_list: Vec::with_capacity(capacity),
            loaded_line_list: Vec::new(),
            completion_progress: state,
        }
    }

    /// List all loaded lines.
    pub fn loaded_line_list(&self) -> impl Iterator<Item = CharGridLine<'_, Self>> {
        let create = |(slice, eol)| CharGridLine::new(slice, eol, self);
        self.loaded_line_list.iter().copied().map(create)
    }

    /// Number of lines.
    pub fn loaded_line_count(&self) -> usize {
        self.loaded_line_list.len()
    }

    /// Whether the grid is completed.
    pub const fn completion(&self) -> CompletionStatus {
        match self.completion_progress {
            Some(_) => CompletionStatus::Incomplete,
            None => CompletionStatus::Complete,
        }
    }

    /// Return the total number of characters if the grid is fully loaded.
    /// * `Some(n)` means that the grid is fully loaded with `n` characters.
    /// * `None` means that the grid isn't yet completed.
    pub const fn total_char_count(&self) -> Option<usize> {
        match self.completion() {
            CompletionStatus::Complete => Some(self.loaded_char_count),
            CompletionStatus::Incomplete => None,
        }
    }

    /// Return reference to the full string if the grid is fully loaded.
    /// * `Some(text)` means that the grid is fully loaded with `text` being the content.
    /// * `None` means that the grid isn't yet completed.
    pub const fn full_text(&self) -> Option<&String> {
        match self.completion() {
            CompletionStatus::Complete => Some(&self.loaded_text),
            CompletionStatus::Incomplete => None,
        }
    }

    /// Return reference to the complete list of lines if the grid is fully loaded.
    /// * `Some(list)` means that the grid is fully loaded with `list` being the complete list of lines.
    /// * `None` means that the grid isn't yet completed.
    pub fn all_lines(&self) -> Option<impl Iterator<Item = CharGridLine<'_, Self>>> {
        match self.completion() {
            CompletionStatus::Complete => Some(self.loaded_line_list()),
            CompletionStatus::Incomplete => None,
        }
    }
}

/// Success value of [`LazyCharGrid::load_char`].
#[derive(Debug, Clone, Copy)]
pub enum LoadCharReport<'a> {
    /// The grid is completed.
    Document,
    /// Complete a line.
    Line {
        def: TextSliceDef,
        value: &'a str,
        eol: EndOfLine,
    },
    /// Get another character.
    Char(char),
}

/// Failure value of [`LazyCharGrid::load_char`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display, Error)]
pub enum LoadCharError<IterError> {
    /// Encounter an invalid character.
    #[display(fmt = "CR is poorly placed, it was before {followed_by} instead of LF")]
    IllPlacedCarriageReturn { followed_by: char },
    /// Error emitted by character iterator.
    IterError(IterError),
}

impl<IterError, CharIter: Iterator<Item = Result<char, IterError>>> LazyCharGrid<CharIter> {
    /// Add another character to the grid.
    pub fn load_char(&mut self) -> Result<LoadCharReport<'_>, LoadCharError<IterError>> {
        let LazyCharGrid {
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
            loaded_char_list.shrink_to_fit(); // The list is final (no more changes), it is safe to shrink to free some memory
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
            Ok(LoadCharReport::Line {
                def: line_slice_def,
                value: line_src_text,
                eol,
            })
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

    /// Load a whole line.
    ///
    /// **Returns:**
    /// * `Ok(Some((slice, text)))` means that a line with the shape of `slice` and content of `text` has been loaded.
    /// * `Ok(None)` means that there are no more line to load (i.e. the grid is completed).
    /// * `Err(error)` means that an error occurred.
    pub fn load_line(
        &mut self,
    ) -> Result<Option<(CharGridLine<'_, Self>, &'_ str)>, LoadCharError<IterError>> {
        loop {
            match self.load_char()? {
                LoadCharReport::Char(_) => continue,
                LoadCharReport::Line { def, value, eol } => {
                    let line = CharGridLine::new(def, eol, self);
                    return Ok(Some((line, value)));
                }
                LoadCharReport::Document => return Ok(None),
            }
        }
    }

    /// Load the whole text.
    pub fn load_all(&mut self) -> Result<(), LoadCharError<IterError>> {
        while self.completion() == CompletionStatus::Incomplete {
            self.load_char()?;
        }
        Ok(())
    }

    /// Load the whole text and return a [`CompletedCharGrid`].
    pub fn into_completed(mut self) -> Result<CompletedCharGrid, LoadCharError<IterError>> {
        self.load_all()?;
        Ok(CompletedCharGrid {
            char_count: self.loaded_char_count,
            text: self.loaded_text,
            char_list: self.loaded_char_list,
            line_list: self.loaded_line_list,
        })
    }
}

impl<CharIter> Debug for LazyCharGrid<CharIter> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let char_count = self.loaded_char_count();
        let line_count = self.loaded_line_count();
        let completion = self.completion();
        write!(
            f,
            "LazyCharGrid of {line_count} lines {char_count} chars ({completion})",
        )
    }
}

impl LazyCharGrid<Infallible> {
    /// Allocating a character grid and assign a stream to load from.
    ///
    /// Unlike [`new()`](LazyCharGrid::new), this constructor takes an infallible iterator.
    ///
    /// **Parameters:**
    /// * `src_char_iter` is an iterator that emits UTF-8 characters.
    /// * `capacity` is the capacity of the final text (e.g. file size of the source code).
    ///
    /// **Example:** Load a stream of characters
    ///
    /// ```rust
    /// # use pretty_assertions::assert_eq;
    /// use egg_grammar::{LazyCharGrid, EndOfLine};
    /// let src_text = "Hello,\r\nI ❤️ Rust 🦀!!\nAnd I program in it.";
    /// let mut grid = LazyCharGrid::new_infallible(src_text.chars(), src_text.len());
    /// grid.load_all().unwrap();
    /// assert_eq!(grid.loaded_text(), src_text);
    /// let lines: Vec<_> = grid
    ///     .loaded_line_list()
    ///     .map(|line| (line.text_without_eol(), line.eol()))
    ///     .collect();
    /// assert_eq!(lines, [
    ///     ("Hello,", EndOfLine::CRLF),
    ///     ("I ❤️ Rust 🦀!!", EndOfLine::LF),
    ///     ("And I program in it.", EndOfLine::EOF),
    /// ]);
    /// ```
    pub fn new_infallible<InfallibleCharIter>(
        src_char_iter: InfallibleCharIter,
        capacity: usize,
    ) -> LazyCharGrid<impl Iterator<Item = Result<char, Infallible>>>
    where
        InfallibleCharIter: IntoIterator<Item = char>,
    {
        let char_iter = src_char_iter.into_iter().map(Result::Ok);
        LazyCharGrid::new(char_iter, capacity)
    }

    /// Start load characters from a string slice.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(
        src_text: &str,
    ) -> LazyCharGrid<impl Iterator<Item = Result<char, Infallible>> + '_> {
        LazyCharGrid::new_infallible(src_text.chars(), src_text.len())
    }
}
