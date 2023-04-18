use super::{ContentToken, IndentToken};
use std::str::Lines;

/// Token scanner.
///
/// Use [the iterator interface](Iterator) to interact with the scanner.
#[derive(Debug)]
pub struct Scan<'a> {
    text: &'a str,
    state: State<'a>,
}

/// State of the scanner.
#[derive(Debug)]
struct State<'a> {
    line_iter: Lines<'a>,
    context: Context,
}

/// Indicate which class of token should the scanner produce.
#[derive(Debug)]
enum Context {
    /// The normal, default state for most types of tokens.
    Outermost,
}

impl<'a> Scan<'a> {
    /// Start scanning text for tokens.
    pub fn new(text: &'a str) -> Self {
        let line_iter = text.lines();
        let context = Context::Outermost;
        let state = State { line_iter, context };
        Scan { text, state }
    }
}

impl<'a> Iterator for Scan<'a> {
    type Item = (IndentToken, Vec<ContentToken<&'a str>>);
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
