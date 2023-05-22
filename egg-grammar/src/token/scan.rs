use super::{
    EndingToken, IndentToken, InvalidToken, MiddleToken, ParseMiddleToken, TokenLine, TokenLineItem,
};
use split_first_char::split_first_char;
use std::str::Lines;

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
    lines: Lines<'a>,
}

impl<'a> Scan<'a> {
    /// Start scanning text for tokens.
    pub fn new(text: &'a str) -> Self {
        let lines = text.lines();
        let state = State { lines };
        Scan { state }
    }
}

impl<'a> Iterator for Scan<'a> {
    type Item = TokenLine<&'a str>;
    fn next(&mut self) -> Option<Self::Item> {
        let Scan { state } = self;
        let State { lines } = state;
        let ln_text = lines.next()?;
        let (indent, rest) = IndentToken::parse(ln_text);
        let indent_src_text = &ln_text[..indent.len()];
        let indent_item = TokenLineItem::new(0, indent_src_text, indent);
        let indent = &indent_item.token; // re-borrow a moved value

        let mut input = rest;
        let mut offset = indent.len();
        let mut middle = Vec::new();

        while !input.is_empty() {
            if let Some(token) = EndingToken::build(indent, input, || lines.next()) {
                middle.shrink_to_fit();
                let ending_item = TokenLineItem::new(offset, input, token);
                let token_line = TokenLine::new(ln_text, indent_item, middle, Some(ending_item));
                return Some(token_line);
            }

            if let Some((token, rest)) = MiddleToken::parse(input) {
                let token_len = input.len() - rest.len();
                let src_text = &input[..token_len];
                middle.push(TokenLineItem::new(offset, src_text, Ok(token)));
                offset += token_len;
                input = rest;
                continue;
            }

            if let Some((char, rest)) = split_first_char(input) {
                let token = InvalidToken(char);
                let char_len = char.len_utf8();
                let src_text = &input[..char_len];
                middle.push(TokenLineItem::new(offset, src_text, Err(token)));
                offset += char_len;
                input = rest;
                continue;
            }

            break;
        }

        middle.shrink_to_fit();
        let token_line = TokenLine::new(ln_text, indent_item, middle, None);
        Some(token_line)
    }
}
