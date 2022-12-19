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
        .all_lines()
        .expect("get the complete list of lines")
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
