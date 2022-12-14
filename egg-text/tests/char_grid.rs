use egg_text::{CharCell, CompletedCharGrid, EndOfLine, IterChar, IterLine, LazyCharGrid};
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
fn char_correctness() {
    let grid = grid();

    eprintln!("\nTEST: NON END-OF-LINE ONLY");
    let non_eol = grid
        .iter_char()
        .map(CharCell::<char>::try_from)
        .filter_map(Result::ok);
    for char_cell in non_eol {
        eprintln!();
        dbg!(char_cell);
        let len_utf8 = dbg!(char_cell.value().len_utf8());
        let offset = dbg!(char_cell.offset_from_doc_start());
        let src_char = dbg!(&SRC_TEXT[offset..(offset + len_utf8)]);
        assert_eq!(char_cell.to_string(), src_char);
    }

    eprintln!("\nTEST: END-OF-LINE ONLY");
    let eol_only = grid
        .iter_char()
        .map(CharCell::<EndOfLine>::try_from)
        .filter_map(Result::ok);
    for char_cell in eol_only {
        eprintln!();
        dbg!(char_cell);
        let len = dbg!(char_cell.value().len());
        let offset = dbg!(char_cell.offset_from_doc_start());
        let src_char = dbg!(&SRC_TEXT[offset..(offset + len)]);
        assert_eq!(char_cell.value().as_ref(), src_char);
    }

    eprintln!("\nTEST: THE WHOLE THING");
    let mut acc = String::new();
    for char_cell in grid.iter_char() {
        acc += char_cell.to_string().as_str();
    }
    assert_eq!(acc, SRC_TEXT);
}

#[test]
fn line_correctness() {
    use EndOfLine::*;
    let grid = grid();
    let received: Vec<_> = grid.iter_line().collect();
    let received: Vec<_> = received
        .iter()
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
