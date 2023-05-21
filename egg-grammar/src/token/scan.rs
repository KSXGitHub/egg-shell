mod ln_num_iter;

use super::{CommentToken, EndingToken, IndentToken, TokenLine};
use egg_ast::ColNum;
use ln_num_iter::LnNumIter;
use pipe_trait::Pipe;

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
        let (indent, ln_text) = IndentToken::parse(ln_text);
        let indent_col = ColNum::from_pred_count(0); // this may be wasteful data, but it helps with symmetry

        let mut col = indent_col.advance_by(indent.len());
        let mut middle = Vec::new();

        while !ln_text.is_empty() {
            if let Some(comment) = CommentToken::parse(ln_text) {
                let token = comment.pipe(EndingToken::from).pipe(Some);
                middle.shrink_to_fit();
                let token_line = TokenLine::new(ln_num, (indent_col, indent), middle, (col, token));
                return Some(token_line);
            }

            todo!()
        }

        middle.shrink_to_fit();
        let token_line = TokenLine::new(ln_num, (indent_col, indent), middle, (col, None));
        Some(token_line)
    }
}
