use egg_grammar::{CharGrid, EndOfLine::*};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::convert::Infallible;

const SRC_TEXT: &str = concat! {
    "Hello,\n",
    "I ❤️ Rust 🦀,\r\n",
    "So I use it to create a programming language,\n",
    "The language is called 'egg-shell' 🥚",
};

fn table() -> CharGrid<impl Iterator<Item = Result<char, Infallible>>> {
    SRC_TEXT
        .pipe(CharGrid::from_str)
        .into_completed()
        .expect("load table")
}

#[test]
fn line_correctness() {
    let table = table();
    let received: Vec<_> = table
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
    let table = table();
    let char_count = table
        .total_char_count()
        .expect("get total number of characters");
    dbg!(char_count);
    let text = table.full_text().expect("get full text");
    eprintln!("FULL TEXT:\n{text}\nEND FULL TEXT");
    eprintln!("TEST: full == loaded");
    assert_eq!(text, table.loaded_text());
    assert_eq!(char_count, table.loaded_char_count());
    eprintln!("TEST: full == source");
    assert_eq!(text, SRC_TEXT);
    assert_eq!(char_count, SRC_TEXT.chars().count());
}

#[test]
fn capacity() {
    let text = "ABC\nDEF\r\nGHI";
    let table = CharGrid::from_str(text);
    let received = (
        table.loaded_text().capacity(),
        table.loaded_char_list().capacity(),
    );
    dbg!(received);
    let expected = (text.len(), text.len() * std::mem::size_of::<char>());
    assert_eq!(received, expected);
}
