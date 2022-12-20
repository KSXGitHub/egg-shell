use egg_grammar::{EndOfLine::*, LazyCharGrid};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::convert::Infallible;

const SRC_TEXT: &str = concat! {
    "Hello,\n",
    "I ❤️ Rust 🦀,\r\n",
    "So I use it to create a programming language,\n",
    "The language is called 'egg-shell' 🥚",
};

fn grid() -> LazyCharGrid<impl Iterator<Item = Result<char, Infallible>>> {
    SRC_TEXT
        .pipe(LazyCharGrid::from_str)
        .into_completed()
        .expect("load grid")
}

#[test]
fn line_correctness() {
    let grid = grid();
    let received: Vec<_> = grid
        .all_lines()
        .expect("get the complete list of lines")
        .map(|line| (line.text_without_eol(), line.eol()))
        .collect();
    dbg!(&received);
    let expected = [
        ("Hello,", LF),
        ("I ❤️ Rust 🦀,", CRLF),
        ("So I use it to create a programming language,", LF),
        ("The language is called 'egg-shell' 🥚", EOF),
    ];
    assert_eq!(received, expected);
}

#[test]
fn text_correctness() {
    let grid = grid();
    let char_count = grid
        .total_char_count()
        .expect("get total number of characters");
    dbg!(char_count);
    let text = grid.full_text().expect("get full text");
    eprintln!("FULL TEXT:\n{text}\nEND FULL TEXT");
    eprintln!("TEST: full == loaded");
    assert_eq!(text, grid.loaded_text());
    assert_eq!(char_count, grid.loaded_char_count());
    eprintln!("TEST: full == source");
    assert_eq!(text, SRC_TEXT);
    assert_eq!(char_count, SRC_TEXT.chars().count());
}

#[test]
fn capacity() {
    let text = "ABC\nDEF\r\nGHI";
    let grid = LazyCharGrid::from_str(text);
    let received = (
        grid.loaded_text().capacity(),
        grid.loaded_char_list().capacity(),
    );
    dbg!(received);
    let expected = (text.len(), text.len() * std::mem::size_of::<char>());
    assert_eq!(received, expected);
}
