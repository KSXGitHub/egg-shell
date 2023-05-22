mod ln_num_iter;

use super::{
    EndingToken, IndentToken, InvalidToken, MiddleToken, ParseMiddleToken, TokenLine, TokenLineItem,
};
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
        let (indent, rest) = IndentToken::parse(ln_text);
        let indent_src_text = &ln_text[..indent.len()];
        let indent_item = TokenLineItem::new(0, indent_src_text, indent);
        let indent = &indent_item.token; // re-borrow a moved value

        let mut ln_text = rest;
        let mut offset = indent.len();
        let mut middle = Vec::new();

        while !ln_text.is_empty() {
            let next_line = || lines.next().map(|(_, text)| text);
            if let Some(token) = EndingToken::build(indent, ln_text, next_line) {
                middle.shrink_to_fit();
                let ending_item = TokenLineItem::new(offset, ln_text, token);
                let token_line =
                    TokenLine::new(ln_num, ln_text, indent_item, middle, Some(ending_item));
                return Some(token_line);
            }

            if let Some((token, rest)) = MiddleToken::parse(ln_text) {
                let token_len = ln_text.len() - rest.len();
                let src_text = &ln_text[..token_len];
                middle.push(TokenLineItem::new(offset, src_text, Ok(token)));
                offset += token_len;
                ln_text = rest;
                continue;
            }

            if let Some((token, rest)) = InvalidToken::take(ln_text) {
                let token_len = token.0.len_utf8();
                let src_text = &ln_text[..token_len];
                middle.push(TokenLineItem::new(offset, src_text, Err(token)));
                offset += token_len;
                ln_text = rest;
                continue;
            }

            break; // InvalidToken::take failing means that ln_text is empty
        }

        middle.shrink_to_fit();
        let token_line = TokenLine::new(ln_num, ln_text, indent_item, middle, None);
        Some(token_line)
    }
}
