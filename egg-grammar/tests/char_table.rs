use egg_grammar::char_table::{CharTable, EndOfLine};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

const SRC_TEXT: &str = concat! {
    "Hello,\n",
    "I ❤️ Rust 🦀,\r\n",
    "So I use to create a programming language,\n",
    "The language is called 'egg-shell' 🥚",
};

fn table() -> CharTable<impl Iterator<Item = char>> {
    SRC_TEXT
        .pipe(CharTable::from_static_str)
        .into_completed()
        .expect("load table")
}

#[test]
fn line_correctness() {
    let table = table();
    let received: Vec<_> = table
        .loaded_line_list()
        .iter()
        .map(|(segment, eol)| (segment.to_string(), eol))
        .collect();
    let received: Vec<_> = received
        .iter()
        .map(|(line, eol)| (line.as_str(), **eol))
        .collect();
    dbg!(&received);
    let expected = [
        ("Hello,", EndOfLine::LF),
        ("I ❤️ Rust 🦀,", EndOfLine::CRLF),
        ("So I use to create a programming language,", EndOfLine::LF),
        ("The language is called 'egg-shell' 🥚", EndOfLine::EOF),
    ];
    assert_eq!(received, expected);
}
