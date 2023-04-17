use super::{ContentToken, IndentToken};

/// Token scanner.
///
/// Use [the iterator interface](Iterator) to interact with the scanner.
#[derive(Debug)]
pub struct Scan<'a> {
    text: &'a str,
    state: State,
}

/// State of the scanner.
#[derive(Debug, Default)]
struct State {
    scanned_chars: usize,
    scanned_len: usize,
    context: Context,
}

/// Indicate which class of token should the scanner produce.
#[derive(Debug, Default)]
enum Context {
    /// The normal, default state for most types of tokens.
    #[default]
    Outermost,
}

impl<'a> Scan<'a> {
    /// Start scanning text for tokens.
    pub fn new(text: &'a str) -> Self {
        let state = State::default();
        Scan { text, state }
    }
}

impl<'a> Iterator for Scan<'a> {
    type Item = (IndentToken, ContentToken<&'a str>);
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
