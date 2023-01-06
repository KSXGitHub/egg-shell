use super::{CompletionStatus, LazyCharGridData, LoadCharError, LoadCharReport, LoadingProgress};
use crate::char_grid::{CharGridLine, CompletedCharGrid};
use parking_lot::{RwLock, RwLockReadGuard};
use pipe_trait::Pipe;
use std::{
    convert::Infallible,
    fmt::{self, Debug, Formatter},
    str::Chars,
};

/// Grid of characters.
pub struct LazyCharGrid<CharIter> {
    /// Inner data of the grid.
    data: RwLock<LazyCharGridData<CharIter>>,
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
        let data = LazyCharGridData {
            loaded_text: String::with_capacity(capacity),
            loaded_char_list: Vec::with_capacity(capacity),
            loaded_line_list: Vec::new(),
            completion_progress: state,
        };
        LazyCharGrid {
            data: RwLock::new(data),
        }
    }

    // Acquire read of the inner data.
    pub fn data(&self) -> RwLockReadGuard<'_, LazyCharGridData<CharIter>> {
        self.data.read()
    }

    /// Total number of loaded characters.
    pub fn loaded_char_count(&self) -> usize {
        self.data().loaded_char_list().len()
    }

    /// Number of lines.
    pub fn loaded_line_count(&self) -> usize {
        self.data().loaded_line_list.len()
    }

    /// Whether the grid is completed.
    pub fn completion(&self) -> CompletionStatus {
        match self.data().completion_progress {
            Some(_) => CompletionStatus::Incomplete,
            None => CompletionStatus::Complete,
        }
    }

    /// Return the total number of characters if the grid is fully loaded.
    /// * `Some(n)` means that the grid is fully loaded with `n` characters.
    /// * `None` means that the grid isn't yet completed.
    pub fn total_char_count(&self) -> Option<usize> {
        match self.completion() {
            CompletionStatus::Complete => Some(self.loaded_char_count()),
            CompletionStatus::Incomplete => None,
        }
    }
}

impl<IterError, CharIter: Iterator<Item = Result<char, IterError>>> LazyCharGrid<CharIter> {
    /// Add another character to the grid.
    pub fn load_char(&self) -> Result<LoadCharReport, LoadCharError<IterError>> {
        self.data.write().load_char()
    }

    /// Load a whole line.
    ///
    /// **Returns:**
    /// * `Ok(Some(slice))` means that a line of `slice` has been loaded.
    /// * `Ok(None)` means that there are no more line to load (i.e. the grid is completed).
    /// * `Err(error)` means that an error occurred.
    pub fn load_line(&self) -> Result<Option<CharGridLine<&'_ Self>>, LoadCharError<IterError>> {
        loop {
            match self.load_char()? {
                LoadCharReport::Char(_) => continue,
                LoadCharReport::Line { def, eol, .. } => {
                    return CharGridLine::new(def, eol, self).pipe(Some).pipe(Ok);
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
        let data = self.data.into_inner();
        Ok(CompletedCharGrid {
            char_count: data.loaded_char_list.len(),
            text: data.loaded_text,
            char_list: data.loaded_char_list,
            line_list: data.loaded_line_list,
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

/// Return type of [`LazyCharGrid::new_infallible`].
pub type LazyCharGridInfallible<SrcCharIter> = LazyCharGrid<InfallibleCharIter<SrcCharIter>>;

impl<SrcCharIter> LazyCharGridInfallible<SrcCharIter>
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
    /// use egg_text::{LazyCharGrid, EndOfLine, TryIterLine};
    /// let src_text = "Hello,\r\nI ❤️ Rust 🦀!!\nAnd I program in it.";
    /// let mut grid = LazyCharGrid::new_infallible(src_text.chars(), src_text.len());
    /// grid.load_all().unwrap();
    /// assert_eq!(grid.data().loaded_text(), src_text);
    /// let mut iter = grid.try_iter_line();
    /// let mut test = |text_without_eol, eol| {
    ///     let line = iter.next().expect("Some").expect("Ok");
    ///     assert_eq!(&line.text_without_eol(), text_without_eol);
    ///     assert_eq!(line.eol(), eol);
    /// };
    /// test("Hello,", EndOfLine::CRLF);
    /// test("I ❤️ Rust 🦀!!", EndOfLine::LF);
    /// test("And I program in it.", EndOfLine::EOF);
    /// ```
    pub fn new_infallible<SrcCharIntoIter>(src_char_iter: SrcCharIntoIter, capacity: usize) -> Self
    where
        SrcCharIntoIter: IntoIterator<IntoIter = SrcCharIter>,
    {
        let char_iter = src_char_iter.into_iter().pipe(InfallibleCharIter);
        LazyCharGrid::new(char_iter, capacity)
    }
}

/// `CharIter` type of [`LazyCharGrid::from_str`].
pub type FromStrCharIter<'a> = InfallibleCharIter<Chars<'a>>;

/// Return type of [`LazyCharGrid::from_str`].
pub type LazyCharGridFromStr<'a> = LazyCharGrid<FromStrCharIter<'a>>;

impl<'a> LazyCharGridFromStr<'a> {
    /// Start load characters from a string slice.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(src_text: &'a str) -> Self {
        LazyCharGrid::new_infallible(src_text.chars(), src_text.len())
    }
}
