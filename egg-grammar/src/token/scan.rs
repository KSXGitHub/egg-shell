mod ln_num_iter;

use super::{EndingToken, IndentToken, MiddleToken, ParseMiddleToken, TokenLine};
use egg_ast::ColNum;
use ln_num_iter::LnNumIter;

/// Token scanner.
///
/// Use [the iterator interface](Iterator) to interact with the scanner.
#[derive(Debug)]
pub struct Scan<'a> {
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
        Scan { state }
    }
}

impl<'a> Iterator for Scan<'a> {
    type Item = TokenLine<&'a str>;
    fn next(&mut self) -> Option<Self::Item> {
        let Scan { state } = self;
        let State { lines } = state;
        let (ln_num, ln_text) = lines.next()?;
        let (indent, mut ln_text) = IndentToken::parse(ln_text);

        let mut col = ColNum::from_pred_count(indent.len());
        let mut middle = Vec::new();

        while !ln_text.is_empty() {
            let next_line = || lines.next().map(|(_, text)| text);
            if let Some(token) = EndingToken::build(&indent, ln_text, next_line) {
                middle.shrink_to_fit();
                let token = Some(token);
                let token_line = TokenLine::new(ln_num, indent, middle, (col, token));
                return Some(token_line);
            }

            if let Some((token, rest)) = MiddleToken::parse(ln_text) {
                let token_len = ln_text.len() - rest.len(); // TODO: switch to unicode width
                col = col.advance_by(token_len);
                middle.push((col, token));
                ln_text = rest;
            }
        }

        middle.shrink_to_fit();
        let token_line = TokenLine::new(ln_num, indent, middle, (col, None));
        Some(token_line)
    }
}
