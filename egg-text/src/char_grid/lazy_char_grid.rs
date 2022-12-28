use super::{CharGridLine, CompletedCharGrid};
use crate::{
    text_slice::ScanText, CharCell, CharCoord, CharOrEol, EndOfLine, LoadCharAt, LoadLineAt,
    Ordinal, TextSliceDef, TryIterLoadChar, TryIterLoadLine,
};
use assert_cmp::debug_assert_op;
use derive_more::{Error, IsVariant};
use getset::{CopyGetters, Getters};
use pipe_trait::Pipe;
use std::{
    cmp::Ordering,
    convert::Infallible,
    fmt::{self, Debug, Formatter},
    str::Chars,
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
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, AsRefStr, IntoStaticStr, IsVariant)]
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
    pub(super) loaded_char_list: Vec<CharCell<char>>,
    /// List of loaded line coordinates.
    #[getset(get = "pub")]
    pub(super) loaded_line_list: Vec<CharGridLine>,
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
    /// use egg_text::LazyCharGrid;
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
    pub fn all_lines(&self) -> Option<&'_ Vec<CharGridLine>> {
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
#[derive(Debug, derive_more::Display, Clone, Copy, PartialEq, Eq, Error)]
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
            loaded_line_list.push(CharGridLine::new(line_slice_def, EndOfLine::EOF));
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
            loaded_line_list.push(CharGridLine::new(line_slice_def, eol));
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
    /// * `Ok(Some(slice))` means that a line of `slice` has been loaded.
    /// * `Ok(None)` means that there are no more line to load (i.e. the grid is completed).
    /// * `Err(error)` means that an error occurred.
    pub fn load_line(&mut self) -> Result<Option<CharGridLine>, LoadCharError<IterError>> {
        loop {
            match self.load_char()? {
                LoadCharReport::Char(_) => continue,
                LoadCharReport::Line { def, eol, .. } => {
                    return CharGridLine::new(def, eol).pipe(Some).pipe(Ok);
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

/// `CharIter` type of [`LazyCharGrid::new_infallible`].
pub struct InfallibleCharIter<SrcCharIter>(SrcCharIter)
where
    SrcCharIter: Iterator<Item = char>;

impl<SrcCharIter> Iterator for InfallibleCharIter<SrcCharIter>
where
    SrcCharIter: Iterator<Item = char>,
{
    type Item = Result<char, Infallible>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Result::Ok)
    }
}

impl<SrcCharIter> LazyCharGrid<InfallibleCharIter<SrcCharIter>>
where
    SrcCharIter: Iterator<Item = char>,
{
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
    /// use egg_text::{LazyCharGrid, EndOfLine};
    /// let src_text = "Hello,\r\nI ❤️ Rust 🦀!!\nAnd I program in it.";
    /// let mut grid = LazyCharGrid::new_infallible(src_text.chars(), src_text.len());
    /// grid.load_all().unwrap();
    /// assert_eq!(grid.loaded_text(), src_text);
    /// let lines: Vec<_> = grid
    ///     .loaded_line_list()
    ///     .iter()
    ///     .map(|line| (line.text_without_eol(&grid), line.eol()))
    ///     .collect();
    /// assert_eq!(lines, [
    ///     ("Hello,", EndOfLine::CRLF),
    ///     ("I ❤️ Rust 🦀!!", EndOfLine::LF),
    ///     ("And I program in it.", EndOfLine::EOF),
    /// ]);
    /// ```
    pub fn new_infallible<SrcCharIntoIter>(src_char_iter: SrcCharIntoIter, capacity: usize) -> Self
    where
        SrcCharIntoIter: IntoIterator<IntoIter = SrcCharIter>,
    {
        let char_iter = src_char_iter.into_iter().pipe(InfallibleCharIter);
        LazyCharGrid::new(char_iter, capacity)
    }
}

impl<'a> LazyCharGrid<InfallibleCharIter<Chars<'a>>> {
    /// Start load characters from a string slice.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(src_text: &'a str) -> Self {
        LazyCharGrid::new_infallible(src_text.chars(), src_text.len())
    }
}

/// Error type of [`LoadCharAt`] for [`LazyCharGrid`].
#[derive(Debug, derive_more::Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum CharAtError<IterError> {
    /// An error occurred while loading a character.
    LoadCharError(LoadCharError<IterError>),
    /// The source iterator doesn't have enough lines to match the requested line index.
    #[display(fmt = "Line does not exist")]
    LineOutOfBound,
    /// The line doesn't have enough characters to match the requested column index.
    #[display(fmt = "Column does not exist")]
    ColumnOutOfBound,
}

impl<'a, IterError, CharIter> LoadCharAt<'a> for LazyCharGrid<CharIter>
where
    CharIter: Iterator<Item = Result<char, IterError>>,
{
    type Char = CharCell<char>; // TODO: change this to CharCell<CharOrEol>
    type Error = CharAtError<IterError>;

    fn load_char_at(&'a mut self, coord: CharCoord) -> Result<CharCell<char>, Self::Error> {
        let line = self.load_line_at(coord.line).map_err(|error| match error {
            LineAtError::LoadCharError(error) => CharAtError::LoadCharError(error),
            LineAtError::OutOfBound => CharAtError::LineOutOfBound,
        })?;
        if coord.column.pred_count() >= line.slice().char_count() {
            return Err(CharAtError::ColumnOutOfBound);
        }
        let char_pos = line
            .slice()
            .first_char_pos()
            .advance_by(coord.column.pred_count());
        self.loaded_char_list()
            .get(char_pos.pred_count())
            .copied()
            .expect("char_pos should be within the range of char_list")
            .pipe(Ok)
    }
}

/// Error type of [`LoadLineAt`] for [`LazyCharGrid`].
#[derive(Debug, derive_more::Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum LineAtError<IterError> {
    /// An error occurred while loading a character.
    LoadCharError(LoadCharError<IterError>),
    /// The source iterator doesn't have enough lines to match the requested index.
    #[display(fmt = "Line does not exist")]
    OutOfBound,
}

impl<'a, IterError, CharIter> LoadLineAt<'a> for LazyCharGrid<CharIter>
where
    CharIter: Iterator<Item = Result<char, IterError>> + 'a,
{
    type Error = LineAtError<IterError>;
    type Line = CharGridLine;
    fn load_line_at(&'a mut self, ln_num: Ordinal) -> Result<Self::Line, Self::Error> {
        while self.loaded_line_list.len() <= ln_num.pred_count()
            && self.completion().is_incomplete()
        {
            self.load_line().map_err(LineAtError::LoadCharError)?;
        }
        if let Some(line) = self.loaded_line_list.get(ln_num.pred_count()) {
            return Ok(*line);
        }
        Err(LineAtError::OutOfBound)
    }
}

/// An iterator that emits instances of [`CharCell`] from [`LazyCharGrid`].
pub struct CharIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    ln_index: Ordinal,
    col_index: Ordinal,
    grid: &'a mut LazyCharGrid<SrcIter>,
}

impl<'a, SrcIterError, SrcIter> Iterator for CharIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Item = Result<CharCell<CharOrEol>, LoadCharError<SrcIterError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = match self.grid.load_line_at(self.ln_index) {
            Err(LineAtError::LoadCharError(error)) => return Some(Err(error)),
            Err(LineAtError::OutOfBound) => return None,
            Ok(line) => line,
        };
        match self.col_index.pred_count().cmp(&line.slice().char_count()) {
            Ordering::Greater => panic!("Column index should never be greater than line count"),
            Ordering::Equal => {
                let coord = CharCoord {
                    line: self.ln_index,
                    column: self.col_index,
                };
                self.ln_index = self.ln_index.advance_by(1);
                self.col_index = Ordinal::from_pred_count(0);
                let offset_from_ln_start = line.slice().size();
                let offset_from_doc_start = line.slice().offset() + line.slice().size();
                let value = CharOrEol::EndOfLine(line.eol());
                let char_cell = CharCell {
                    coord,
                    offset_from_ln_start,
                    offset_from_doc_start,
                    value,
                };
                Some(Ok(char_cell))
            }
            Ordering::Less => {
                let char_pos = line
                    .slice()
                    .first_char_pos()
                    .advance_by(self.col_index.pred_count());
                self.col_index = self.col_index.advance_by(1);
                self.grid
                    .loaded_char_list()
                    .get(char_pos.pred_count())
                    .copied()?
                    .map(CharOrEol::from)
                    .pipe(Ok)
                    .pipe(Some)
            }
        }
    }
}

impl<'a, SrcIterError, SrcIter> TryIterLoadChar<'a> for LazyCharGrid<SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Char = CharCell<CharOrEol>;
    type Error = LoadCharError<SrcIterError>;
    type CharResultLoadIter = CharIter<'a, SrcIterError, SrcIter>;

    fn try_iter_load_char(&'a mut self) -> Self::CharResultLoadIter {
        CharIter {
            ln_index: Ordinal::from_pred_count(0),
            col_index: Ordinal::from_pred_count(0),
            grid: self,
        }
    }
}

/// An iterator that emits instances of [`CharGridLine`] from [`LazyCharGrid`].
pub struct LineIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    index: Ordinal,
    grid: &'a mut LazyCharGrid<SrcIter>,
}

impl<'a, SrcIterError, SrcIter> Iterator for LineIter<'a, SrcIterError, SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Item = Result<CharGridLine, LoadCharError<SrcIterError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index = index.advance_by(1);
        let line = self
            .grid
            .load_line_at(Ordinal::from_pred_count(index.pred_count()));
        match line {
            Err(LineAtError::LoadCharError(error)) => Some(Err(error)),
            Err(LineAtError::OutOfBound) => None,
            Ok(line) => Some(Ok(line)),
        }
    }
}

impl<'a, SrcIterError, SrcIter> TryIterLoadLine<'a> for LazyCharGrid<SrcIter>
where
    SrcIterError: 'a,
    SrcIter: Iterator<Item = Result<char, SrcIterError>> + 'a,
{
    type Line = CharGridLine;
    type Error = LoadCharError<SrcIterError>;
    type LineResultLoadIter = LineIter<'a, SrcIterError, SrcIter>;

    fn try_iter_load_line(&'a mut self) -> Self::LineResultLoadIter {
        LineIter {
            index: Ordinal::from_pred_count(0),
            grid: self,
        }
    }
}
