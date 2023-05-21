mod ln_num_iter;

use super::TokenLine;
use ln_num_iter::LnNumIter;

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
    lines: LnNumIter<'a>,
}

impl<'a> Scan<'a> {
    /// Start scanning text for tokens.
    pub fn new(text: &'a str) -> Self {
        let lines = LnNumIter::new(text);
        let state = State { lines };
        Scan { text, state }
    }
}

impl<'a> Iterator for Scan<'a> {
    type Item = TokenLine<&'a str>;
    fn next(&mut self) -> Option<Self::Item> {
        let Scan { text, state } = self;
        let State { lines } = state;
        let (ln_num, ln_text) = lines.next()?;

        todo!()
    }
}
