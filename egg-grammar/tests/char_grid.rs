use egg_grammar::{CompletedCharGrid, EndOfLine::*, LazyCharGrid};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

const SRC_TEXT: &str = concat! {
    "Hello,\n",
    "I ❤️ Rust 🦀,\r\n",
    "So I use it to create a programming language,\n",
    "The language is called 'egg-shell' 🥚",
};

fn grid() -> CompletedCharGrid {
    SRC_TEXT
        .pipe(LazyCharGrid::from_str)
        .into_completed()
        .expect("load grid")
}

#[test]
fn line_correctness() {
    let grid = grid();
    let received: Vec<_> = grid
        .line_list()
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
    let char_count = grid.char_count();
    dbg!(char_count);
    let text = grid.text();
    eprintln!("FULL TEXT:\n{text}\nEND FULL TEXT");
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
